use crate::batchy_service::*;
use crate::util::*;
use std::future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{sleep, spawn};
use std::time::Duration;

const MAX_BLOCK_SIZE: usize = 10;
type Buffer<Req, Res> = Arc<Mutex<Vec<ClientRequest<Req, Res>>>>;

pub struct AutoBatcher<T: BatchyService + 'static> {
    buffer: Buffer<T::Request, T::Response>,
    service: T,
}

impl<T: BatchyService + 'static> AutoBatcher<T> {
    pub fn new(service: T) -> Self {
        let buffer = Arc::new(Mutex::new(vec![]));
        {
            let workers_buffer = Arc::clone(&buffer);
            let workers_service = service.clone();
            spawn(move || {
                let waiter: RecurringWaiter = RecurringWaiter::new(Duration::from_secs(5));
                let workers_service = workers_service;
                loop {
                    sleep(Duration::from_millis(250));
                    if waiter.ready() {
                        Self::make_request_now(
                            Arc::clone(&workers_buffer),
                            workers_service.clone(),
                        );
                    }
                }
            })
        };
        Self { buffer, service }
    }

    fn make_request_now(buffer: Buffer<T::Request, T::Response>, service: T) {
        let current_request_block: Vec<ClientRequest<_, _>> = {
            let mut buffer = buffer.lock().unwrap();
            buffer.drain(..).collect()
        };
        spawn(move || {
            let (stuff_to_request, targets): (Vec<_>, Vec<_>) = current_request_block
                .into_iter()
                .map(|cr: ClientRequest<_, _>| (cr.requested_object, cr.eventual_response))
                .unzip();
            let response = service.batch_call(&stuff_to_request);
            for (target, response) in targets.into_iter().zip(response) {
                target.lock().unwrap().emplace(response);
            }
        });
    }

    pub fn request(
        &mut self,
        rq: T::Request,
    ) -> impl future::Future<Output = T::Response> + Unpin + use<T> {
        let target = Arc::new(Mutex::new(Expecter::default()));
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(ClientRequest {
            requested_object: rq,
            eventual_response: Arc::clone(&target),
        });
        let out = AutobatchWaiter {
            response: Arc::clone(&target),
        };
        if buffer.len() >= MAX_BLOCK_SIZE {
            drop(buffer);
            Self::make_request_now(Arc::clone(&self.buffer), self.service.clone())
        }
        out
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
