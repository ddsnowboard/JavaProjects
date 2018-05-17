use std::thread::JoinHandle;
use std::sync::{Mutex, Arc, Condvar};
use std::collections::VecDeque;
use std::thread;


fn main() {
    const MAX: u64 = 1000000;
    // It's pretty much same speed as C. Which is impressive. Can we build a threadpool and somehow
    // get better performance? Let's find out.
    for i in 1..MAX {
        collatz(i);
    }
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
fn collatz(i: u64) -> bool
{
    let originali = i;
    let mut i = i;
    if i == 0  {
        return true;
    }
    // These are to make sure that we aren't just going in a circle.
    let mut newest = 0;
    let mut second_newest = 0;
    while i != 1
    {
        if i % 2 == 0  {
            i /= 2;
        } else if i % 2 != 0  {
            i = 3 * i + 1;
        } else {
            println!("Something really bad just happened...");
            return false;
        }

        if i == newest || i == second_newest  {
            println!("Doesn't work for {}", originali);
            return false;
        } else {
            second_newest = newest;
            newest = i;
        }
    }
    return true;
}

struct ThreadPool<A: Send> {
    thread_count: usize,
    q: Arc<Mutex<VecDeque<Message<(fn(A), A)>>>>,
    cv: Arc<Condvar>,
    threads: Vec<JoinHandle<()>>
}

enum Message<T: Send> {
    Stop,
    Msg(T)
}

impl<A: 'static + Send> ThreadPool<A> {
    // HEY DUMBASS! YOU KNOW YOU CAN JUST WRAP THE RECV END OF A CHANNEL IN A ARC<MUTEX>, RIGHT?
    fn new(thread_count: usize) -> ThreadPool<A> {
        let mut pool = ThreadPool {
            thread_count: thread_count,
            q: Arc::new(Mutex::new(VecDeque::with_capacity(thread_count * 4))),
            cv: Arc::new(Condvar::new()),
            threads: Vec::with_capacity(thread_count)
        };

        for _ in 0..thread_count {
            let m = Arc::clone(&pool.q);
            let cv = Arc::clone(&pool.cv);
            let fun = || {
                let m = m;
                let cv = cv;
                let mut q = m.lock().unwrap();
                let mut pop = || {
                    let myQueue = &q;
                    while myQueue.is_empty() {
                        myQueue = &cv.wait(currQueue).unwrap();
                    }
                    let out = q.pop_front().unwrap();
                    out
                };

                loop {
                    let curr: Message<(fn(A), A)> = pop();
                    match curr {
                        Message::Stop => break,
                        Message::Msg((func, args)) => func(args)
                    };
                }
            };
            pool.threads.push(thread::spawn(fun));
        }
        pool
    }
}
