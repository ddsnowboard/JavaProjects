use crate::batchy_service::*;
use crate::util::*;
use std::future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll};
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
                                let _ = target.write().unwrap().insert(response);
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
        let target = Arc::new(RwLock::new(Option::None));
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
    response: Arc<RwLock<Option<Res>>>,
}

impl<T> Future for AutobatchWaiter<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let handle = self.response.read().unwrap();
        if handle.is_some() {
            drop(handle);
            let mut handle = self.response.write().unwrap();
            Poll::Ready(handle.take().unwrap())
        } else {
            Poll::Pending
        }
    }
}

struct ClientRequest<Req, Res> {
    requested_object: Req,
    eventual_response: Arc<RwLock<Option<Res>>>,
}
