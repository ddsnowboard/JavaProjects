use crate::batchy_service::*;
use crate::util::*;
use std::future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

pub struct AutoBatcher<T: BatchyService + 'static> {
    buffer: Arc<Mutex<Vec<ClientRequest<T::Request, T::Response>>>>,
    worker_thread: JoinHandle<()>,
}

impl<T: BatchyService + 'static> AutoBatcher<T> {
    pub fn new(service: T) -> Self {
        let buffer = Arc::new(Mutex::new(vec![]));
        let worker_thread = {
            let workers_buffer = Arc::clone(&buffer);
            spawn(move || {
                let should_dump = RecurringWaiter::new(Duration::from_secs(5));
                let service = service;
                loop {
                    sleep(Duration::from_millis(250));
                    if should_dump.ready() {
                        let current_request_block: Vec<ClientRequest<_, _>> = {
                            let mut buffer = workers_buffer.lock().unwrap();
                            buffer.drain(..).collect()
                        };
                        let new_service = service.clone();
                        spawn(move || {
                            let (stuff_to_request, targets): (Vec<_>, Vec<_>) =
                                current_request_block
                                    .into_iter()
                                    .map(|cr: ClientRequest<_, _>| {
                                        (cr.requested_object, cr.eventual_response)
                                    })
                                    .unzip();
                            let response = new_service.batch_call(&stuff_to_request);
                            for (target, response) in targets.into_iter().zip(response) {
                                // This returns a mutable reference for some reason
                                 target.lock().unwrap().emplace(response);
                            }
                        });
                    }
                }
            })
        };
        Self {
            buffer,
            worker_thread,
        }
    }
    pub fn request(
        &mut self,
        rq: T::Request,
    ) -> impl future::Future<Output = T::Response> + Unpin + use<T> {
        let target = Arc::new(Mutex::new(Expecter::default()));
        self.buffer.lock().unwrap().push(ClientRequest {
            requested_object: rq,
            eventual_response: Arc::clone(&target),
        });
        AutobatchWaiter {
            response: Arc::clone(&target),
        }
    }
}

struct AutobatchWaiter<Res> {
    response: Arc<Mutex<Expecter<Res>>>,
}

impl<T> Future for AutobatchWaiter<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut handle = self.response.lock().unwrap();
        if let Some(o) = handle.take() {
            Poll::Ready(o)
        } else {
            handle.register_waker(cx.waker());
            Poll::Pending
        }
    }
}

struct ClientRequest<Req, Res> {
    requested_object: Req,
    eventual_response: Arc<Mutex<Expecter<Res>>>,
}

struct Expecter<T> {
    item: Option<T>,
    waker: Waker,
}

impl<T> Default for Expecter<T> {
    fn default() -> Self {
        Self {
            item: None,
            waker: Waker::noop().clone(),
        }
    }
}

impl<T> Expecter<T> {
    fn emplace(&mut self, i: T) {
        // Why does this return something?
        let _ = self.item.insert(i);
        self.waker.wake_by_ref()
    }

    fn has_item(&self) -> bool {
        self.item.is_some()
    }

    fn take(&mut self) -> Option<T> {
        self.item.take()
    }

    fn register_waker(&mut self, w: &Waker) {
        self.waker.clone_from(w);
    }
}
