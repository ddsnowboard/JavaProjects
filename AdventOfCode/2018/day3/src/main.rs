extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

use regex::Regex;

fn main() {
    let claims: Vec<Claim> = read_input();
    let mut squares: HashMap<(u32, u32), u32> = HashMap::new();
    for c in claims {
        let xs: Vec<_> = (c.x_start..c.x_start + c.x_width).collect();
        let ys: Vec<_> = (c.y_start..c.y_start + c.y_height).collect();
        for &x in &xs {
            for &y in &ys {
                let ct = squares.entry((x, y)).or_insert(0);
                *ct += 1;
            }
        }
    }
    let count = squares.iter()
        .filter(|(_, &ct)| ct > 1)
        .count();
    println!("Found {}", count);
}

struct Claim {
    id: u32,
    x_start: u32,
    y_start: u32,
    x_width: u32,
    y_height: u32
}

fn read_input() -> Vec<Claim> {
    const INPUT_FILE: &str = "input.txt";
    let pth = Path::new(INPUT_FILE);
    let file = File::open(pth).unwrap();
    let lines = BufReader::new(file).lines();
    let re = Regex::new(r"#(?P<id>\d{1,4}) @ (?P<xstart>\d{1,4}),(?P<ystart>\d{1,4}): (?P<xwidth>\d{1,4})x(?P<yheight>\d{1,4})").unwrap();
    lines.map(|s| {
        let s = s.unwrap();
        if let Some(caps) = re.captures(&s) {
            Claim { 
                id: caps.name("id").unwrap().as_str().parse().unwrap(),
                x_start: caps.name("xstart").unwrap().as_str().parse().unwrap(),
                y_start: caps.name("ystart").unwrap().as_str().parse().unwrap(),
                x_width: caps.name("xwidth").unwrap().as_str().parse().unwrap(),
                y_height: caps.name("yheight").unwrap().as_str().parse().unwrap()
            }
        } else {
            panic!("Something bad happened; we didn't match {}", s);
        }
    }).collect()
}
