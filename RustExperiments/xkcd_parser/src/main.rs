extern crate curl;

use curl::easy::Easy;
use std::env;
use std::io::stdout;
use std::io::Write;

fn main() {
    let mut easy = Easy::new();
    let arguments = env::args().collect::<Vec<String>>();
    if arguments.len() == 1 {
        easy.url("http://xkcd.com").unwrap();
    }
    else {
        let comic_number : u32 = arguments[1].trim().parse().expect("You have to give the comic number as the input");
        easy.url(format!("http://xkcd.com/{}", comic_number).as_ref()).unwrap();
    }
    let mut text = Vec::new();
    easy.write_function(|data| {
        let v = &text;
        v.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    for i in text {
        println!("{}", i);
    }
}
