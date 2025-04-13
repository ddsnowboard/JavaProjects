mod autobatcher;
mod batchy_service;
mod util;
use crate::autobatcher::*;
use crate::batchy_service::*;
use crate::util::*;
use std::pin::pin;
use std::task::{Context, Waker};
use std::thread;
use std::time::Duration;

fn main() {
    let mut ab = AutoBatcher::new(Doubler {});
    let mut futures: Vec<_> = (1..30).map(|n| ab.request(n)).collect();
    loop {
        thread::sleep(Duration::from_millis(500));
        for (idx, f) in futures.iter_mut().enumerate() {
            let poll_result = Future::poll(pin!(f), &mut Context::from_waker(Waker::noop()));
            println!("{}: {:?}", idx, poll_result);
        }
    }
}
