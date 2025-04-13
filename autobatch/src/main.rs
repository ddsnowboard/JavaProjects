mod autobatcher;
mod batchy_service;
mod util;
use crate::autobatcher::*;
use crate::batchy_service::*;

#[tokio::main]
async fn main() {
    let mut ab = AutoBatcher::new(Doubler {});
    let futures: Vec<_> = (1..30).map(|n| ab.request(n)).collect();
    for f in futures.into_iter() {
        println!("{}", f.await);
    }
}
