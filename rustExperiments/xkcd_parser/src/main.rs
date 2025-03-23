extern crate curl;
extern crate regex;
extern crate htmlescape;

use regex::Regex;
use curl::easy::Easy;
use std::env;
use htmlescape::decode_html;

fn main() {
    let mut easy = Easy::new();
    easy.follow_location(true).unwrap();
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() == 1 {
        easy.url("http://xkcd.com").unwrap();
    }
    else {
        let comic_number: u32 = arguments[1].trim().parse().expect("You have to give the comic number as the input");
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
    let s: String = text.iter()
        .map(|c| match std::char::from_u32(*c as u32){ Some('\n') => ' ', Some(c) => c, None=> ' '})
        .collect();

    let r = Regex::new("<img src=\"//imgs\\.xkcd\\.com/comics/.*?\" title=\"(.*?)\"").unwrap();
    for cap in r.captures_iter(&s)
    {
        println!("{}", decode_html(cap.at(1).unwrap()).unwrap());
    }
}
