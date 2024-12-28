use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub struct LogMessage {
    source: String,
    message: String,
    data: String,
}

pub trait LogSink: Send {
    fn write(&mut self, message: &LogMessage);
}

#[derive(Clone)]
pub struct Logger {
    sender: Sender<LogMessage>,
}

impl Logger {
    pub fn new(mut log_sink: impl LogSink + 'static) -> Self {
        let (sender, receiver) = channel();
        let _log_catcher = thread::spawn(move || {
            receiver.iter().for_each(|lm| {
                log_sink.write(&lm);
            });
        });
        Self { sender }
    }

    pub fn log(&self, source: String, message: String, data: impl Serialize) {
        let lm = LogMessage {
            source,
            message,
            data: serde_json::to_string(&data).unwrap(),
        };
        self.sender.send(lm).unwrap();
    }
}

#[derive(Default)]
pub struct Stdout {}

impl LogSink for Stdout {
    fn write(&mut self, message: &LogMessage) {
        println!("{}, {}, {}", message.source, message.message, message.data);
    }
}

#[derive(Default)]
pub struct NoOp {}
impl LogSink for NoOp {
    fn write(&mut self, _message: &LogMessage) {}
}
