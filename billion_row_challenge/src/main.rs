#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Lines;
use std::io::Write;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^([\p{L},.'() -]+);([0-9.-]+)$").unwrap();
}

const FILENAME: &str = "measurements.txt";

type Temp = f64;

struct Row {
    city: String,
    temp: Temp,
}

struct City {
    temps: Vec<Temp>,
}

impl City {
    fn new() -> Self {
        City { temps: vec![] }
    }
    fn process_temp(&mut self, temp: Temp) {
        self.temps.push(temp);
    }

    fn get_min(&self) -> Temp {
        self.temps.iter().fold(Temp::INFINITY, |a, &b| a.min(b))
    }

    fn get_max(&self) -> Temp {
        self.temps.iter().fold(-Temp::INFINITY, |a, &b| a.max(b))
    }

    fn get_mean(&self) -> Temp {
        let sum: Temp = self.temps.iter().sum();
        sum / (self.temps.len() as Temp)
    }

    fn to_str(&self, name: &str) -> String {
        format!(
            "{}={:.1}/{:.1}/{:.1}",
            name,
            self.get_min(),
            self.get_mean(),
            self.get_max()
        )
    }
}

fn main() {
    let output = File::create("output.txt").unwrap();
    let writer = BufWriter::new(output);
    write_cities(writer);
}

fn get_cities() -> Vec<(String, City)> {
    let mut cities: HashMap<String, City> = HashMap::new();
    let rows = read_file().map(|r| r.unwrap()).map(read_row);
    rows.for_each(|row| {
        let city = cities.entry(row.city).or_insert_with(City::new);
        city.process_temp(row.temp);
    });
    let mut cities: Vec<_> = cities.into_iter().collect();
    cities.sort_unstable_by_key(|(c, _)| c.clone());
    cities
}

fn write_cities<W: std::io::Write>(mut writer: BufWriter<W>) {
    let cities = get_cities();
    let strings: Vec<_> = cities
        .into_iter()
        .map(|(name, city)| city.to_str(&name))
        .collect();
    writer.write_all(b"{").unwrap();
    writer.write_all(strings.join(", ").as_bytes()).unwrap();
    writer.write_all(b"}\n").unwrap();
}

fn read_row(row: String) -> Row {
    let captures = LINE_REGEX.captures(&row).unwrap();
    Row {
        city: captures.get(1).unwrap().as_str().to_owned(),
        temp: captures.get(2).unwrap().as_str().parse().unwrap(),
    }
}

fn read_file() -> Lines<BufReader<File>> {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}
