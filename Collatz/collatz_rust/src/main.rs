use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut pool = ThreadPool::new(4);
    const MAX: u64 = 1000000;
    // It's pretty much same speed as C. Which is impressive. Can we build a threadpool and somehow
    // get better performance? Let's find out.
    for i in 1..MAX {
        pool.run(collatz, i);
    }

    pool.await();
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
fn collatz(i: u64) {
    let original_i = i;
    let mut i = i;
    if i == 0 {
        return;
    }
    // These are to make sure that we aren't just going in a circle.
    let mut newest = 0;
    let mut second_newest = 0;
    while i != 1 {
        if i % 2 == 0 {
            i /= 2;
        } else if i % 2 != 0 {
            i = 3 * i + 1;
        } else {
            println!("Something really bad just happened...");
        }

        if i == newest || i == second_newest {
            println!("Doesn't work for {}", original_i);
        } else {
            second_newest = newest;
            newest = i;
        }
    }
    // println!("Finished with {}", original_i);
}

struct ThreadPool<A: 'static + Send> {
    thread_count: usize,
    send: Sender<Message<(fn(A), A)>>,
    recv: Arc<Mutex<Receiver<Message<(fn(A), A)>>>>,
    threads: Vec<JoinHandle<()>>,
}

enum Message<T: 'static + Send> {
    Stop,
    Msg(T),
}

impl<A: 'static + Send> ThreadPool<A> {
    fn new(thread_count: usize) -> Self {
        let (send, recv) = channel();
        let mut pool = ThreadPool {
            thread_count: thread_count,
            recv: Arc::new(Mutex::new(recv)),
            send,
            threads: Vec::with_capacity(thread_count),
        };

        for _ in 0..thread_count {
            let mtex = Arc::clone(&pool.recv);
            let fun = move || loop {
                let datum: Message<(fn(A), A)>;
                {
                    let recv = mtex.lock().unwrap();
                    datum = recv.recv().unwrap();
                }
                match datum {
                    Message::Stop => break,
                    Message::Msg((fun, payload)) => fun(payload),
                };
            };
            pool.threads.push(thread::spawn(fun));
        }
        pool
    }

    fn run(&mut self, func: fn(A), args: A) {
        if let Err(e) = self.send.send(Message::Msg((func, args))) {
                println!("Something bad happened; {:?}", e);
        }
    }
    
    fn await(self) {
        for _ in 0..self.thread_count {
            if let Err(e) = self.send.send(Message::Stop) {
                println!("Something bad happened; {:?}", e);
            }
        }

        for thread in self.threads.into_iter() {
            let _  = thread.join();
        }
    }

    fn force_close(self) {
        for _ in 0..self.thread_count {
            if let Err(e) = self.send.send(Message::Stop) {
                println!("Something bad happened; {:?}", e);
            }
        }
    }
}
