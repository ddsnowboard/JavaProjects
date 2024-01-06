use fxhash::FxHashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

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
    let output = File::create("/dev/null").unwrap();
    let writer = BufWriter::new(output);
    write_cities(writer);
}

fn get_cities() -> Vec<(String, City)> {
    let mut cities = FxHashMap::default();
    let mut file = read_file();
    let mut row_holder = String::with_capacity(128);
    loop {
        row_holder.clear();
        match file.read_line(&mut row_holder).unwrap() {
            0 => break,
            _ => {
                let row = read_row(&row_holder);
                let city = cities.entry(row.city).or_insert_with(City::new);
                city.process_temp(row.temp);
            }
        }
    }
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

// This is taking 40% of the time. I can easily parallelize this.
// I can send the output Rows through a channel and add them to the HashMap sequentially,
// or I can figure out how to add stuff to the HashMap in parallel too.
fn read_row(row: &str) -> Row {
    let mut city = String::with_capacity(128);
    let mut temp = String::with_capacity(128);
    let mut it = row.chars();
    for c in it.by_ref() {
        if c != ';' {
            city.push(c);
        } else {
            break;
        }
    }
    for c in it {
        if c != '\n' {
            temp.push(c);
        } else {
            break;
        }
    }
    Row {
        city,
        temp: temp.parse().unwrap(),
    }
}

fn read_file() -> BufReader<File> {
    let file = File::open(FILENAME).unwrap();
    BufReader::new(file)
}
