extern crate curl;

use curl::easy::Easy;
use std::env;

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
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            text.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let s: String = text.iter().map(|c| match std::char::from_digit(*c as u32, 10){ Some(c) => c, None=> ' '}).collect();
    println!("{}", s);
}
