use std::cell::RefCell;
use std::time::{Duration, Instant};

pub struct RecurringWaiter {
    start_time: RefCell<Instant>,
    period: Duration,
}

impl RecurringWaiter {
    pub fn new(period: Duration) -> Self {
        Self {
            start_time: RefCell::new(Instant::now()),
            period,
        }
    }

    pub fn ready(&self) -> bool {
        let mut start_time = self.start_time.borrow_mut();
        if start_time.elapsed() >= self.period {
            *start_time = Instant::now();
            true
        } else {
            false
        }
    }
}
