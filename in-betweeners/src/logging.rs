use rusqlite::{Connection, OpenFlags, Result};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{sync_channel, SyncSender};
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
    sender: SyncSender<LogMessage>,
}

impl Logger {
    pub fn new(mut log_sink: impl LogSink + 'static) -> Self {
        let (sender, receiver) = sync_channel(1_000_000);
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

fn stringify(lm: &LogMessage) -> String {
    format!("{}, {}, {}\n", lm.source, lm.message, lm.data)
}

#[derive(Default)]
pub struct Stdout {}

impl LogSink for Stdout {
    fn write(&mut self, message: &LogMessage) {
        print!("{}", stringify(message));
    }
}

#[derive(Default)]
pub struct NoOp {}
impl LogSink for NoOp {
    fn write(&mut self, _message: &LogMessage) {}
}

pub struct FileSink {
    file: File,
}

impl FileSink {
    pub fn new(filename: &str) -> Self {
        let path = Path::new(filename);
        let f = File::create(path)
            .unwrap_or_else(|e| panic!("{} was not a valid path; error was {}", filename, e));
        Self { file: f }
    }
}

impl LogSink for FileSink {
    fn write(&mut self, message: &LogMessage) {
        self.file
            .write_all(stringify(message).as_bytes())
            .expect("Could not log; Yikes!")
    }
}

struct SqliteSink {
    conn: Connection,
    // I want to keep a prepared statement here, but that has to refer to the connection. How can I
    // do that?
}
impl SqliteSink {
    fn new(path: &str) -> Self {
        let conn = Connection::open(path)
            .unwrap_or_else(|e| panic!("Couldn't open sqlite database; {}", e));
        conn.execute("CREATE TABLE IF NOT EXISTS logs (source TEXT, message TEXT, data TEXT)")
            .unwrap();

        Self { conn }
    }
}
