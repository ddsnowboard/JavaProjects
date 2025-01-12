use duckdb::Connection as DDBConnection;
use rusqlite::Connection;
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

pub struct SqliteSink {
    conn: Connection,
    current_block: Vec<[String; 3]>,
}

impl SqliteSink {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path)
            .unwrap_or_else(|e| panic!("Couldn't open sqlite database; {}", e));
        conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (source TEXT, message TEXT, data TEXT)",
            (),
        )
        .unwrap();

        Self {
            conn,
            current_block: vec![],
        }
    }

    fn maybe_write_block(&mut self) {
        const BLOCK_SIZE: usize = 5000;
        if self.current_block.len() >= BLOCK_SIZE {
            self.write_block();
        }
    }

    fn write_block(&mut self) {
        let tx = self.conn.transaction().unwrap();
        {
            let mut statement = tx
                .prepare_cached(
                    "INSERT INTO logs (source, message, data) 
        VALUES (?1, ?2, ?3)",
                )
                .unwrap();
            self.current_block.drain(..).for_each(|block| {
                statement.execute(block).unwrap();
            });
        }
        tx.commit().unwrap();
    }
}

impl LogSink for SqliteSink {
    fn write(&mut self, message: &LogMessage) {
        self.current_block.push([
            message.source.clone(),
            message.message.clone(),
            message.data.clone(),
        ]);
        self.maybe_write_block()
    }
}

impl Drop for SqliteSink {
    fn drop(&mut self) {
        self.write_block();
    }
}

pub struct DuckDbSink {
    conn: DDBConnection,
    current_block: Vec<[String; 3]>,
}

impl DuckDbSink {
    pub fn new(filename: &str) -> Self {
        let connection = DDBConnection::open(filename).unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS logs (source TEXT, message TEXT, data JSON)",
                [],
            )
            .unwrap();
        Self {
            conn: connection,
            current_block: vec![],
        }
    }

    fn maybe_write_block(&mut self) {
        const BLOCK_SIZE: usize = 100000;
        if self.current_block.len() >= BLOCK_SIZE {
            self.write_block();
        }
    }

    fn write_block(&mut self) {
        let mut appender = self.conn.appender("logs").unwrap();
        self.current_block.drain(..).for_each(|block| {
            appender.append_rows([block]).unwrap();
        });
    }
}

impl LogSink for DuckDbSink {
    fn write(&mut self, message: &LogMessage) {
        self.current_block.push([
            message.source.clone(),
            message.message.clone(),
            message.data.clone(),
        ]);
        self.maybe_write_block()
    }
}

impl Drop for DuckDbSink {
    fn drop(&mut self) {
        self.write_block();
    }
}
