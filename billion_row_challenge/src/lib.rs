use fxhash::FxHashMap;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
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

fn get_cities(file: File) -> Vec<(String, City)> {
    let mut cities = FxHashMap::default();
    let mut row_holder = String::with_capacity(128);
    let mut file = BufReader::new(file);
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

fn split_up_file(
    mut file: File,
    chunk_index: u32,
    total_chunks: u32,
) -> FilePortion<BufReader<File>> {
    file.rewind().unwrap();
    let n_lines = {
        let file = file.try_clone().unwrap();
        let mut reader = BufReader::new(file);
        let mut out: u32 = 0;
        loop {
            let buf = reader.fill_buf().unwrap();
            if buf.is_empty() {
                break;
            }
            for c in buf {
                if *c == b'\n' {
                    out += 1;
                }
            }
            let bytes_processed = buf.len();
            reader.consume(bytes_processed);
        }
        out
    };

    let expected_lines_per_chunk = n_lines / total_chunks;
    let lines_for_this_chunk = if chunk_index == total_chunks - 1 {
        expected_lines_per_chunk + (n_lines % total_chunks)
    } else {
        expected_lines_per_chunk
    };
    println!(
        "Found {} lines for chunk {}",
        lines_for_this_chunk, chunk_index
    );
    let min_offset = chunk_index * expected_lines_per_chunk;
    file.seek(SeekFrom::Start(min_offset.into())).unwrap();
    FilePortion::new(BufReader::new(file), lines_for_this_chunk as u64)
}

pub fn write_cities<W: std::io::Write>(mut writer: BufWriter<W>) {
    let n_threads = 8;
    let file_handles: Vec<_> = (0..n_threads)
        .collect::<Vec<u32>>()
        .into_par_iter()
        .map(|idx| split_up_file(get_file(), idx, n_threads))
        .collect();
    let file = get_file();
    let cities = get_cities(file);
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
// Also I could speed up the float parsing by using fixed-point or something. EDIT Done it's faster
// Also I could just split the whole file into pieces and then give each piece to a thread.
// Got options.
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
        temp: dumb_parse_number(&temp),
    }
}

fn get_file() -> File {
    File::open(FILENAME).unwrap()
}

pub fn dumb_parse_number(s: &str) -> Temp {
    let mut sign: bool = true;
    let mut builder: Temp = 0.0;
    let mut iter = s.chars();
    fn get_output(value: Temp, sign: bool) -> Temp {
        if !sign {
            value * -1.0
        } else {
            value
        }
    }
    for c in iter.by_ref() {
        if c == '-' {
            sign = false;
        } else if c.is_ascii_digit() {
            builder *= 10.0;
            builder += c.to_digit(10).unwrap() as Temp;
        } else if c == '.' {
            break;
        } else {
            return get_output(builder, sign);
        }
    }
    // If we get here we're after the decimal
    let mut place: Temp = 10.0;
    for c in iter {
        if c.is_ascii_digit() {
            builder += (c.to_digit(10).unwrap() as Temp) / (place as Temp);
            place *= 10.0;
        } else {
            break;
        }
    }
    get_output(builder, sign)
}

struct FilePortion<T: BufRead> {
    reader: T,
    max_offset_exclusive: u64,
    current_offset: u64,
}

impl<T: BufRead> FilePortion<T> {
    fn new(reader: T, max_offset_exclusive: u64) -> Self {
        FilePortion {
            reader,
            current_offset: 0,
            max_offset_exclusive,
        }
    }
    fn remaining_bytes(&self) -> u64 {
        self.max_offset_exclusive - self.current_offset - 1
    }
}

impl<T: BufRead> Read for FilePortion<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining_bytes = self.remaining_bytes() as usize;
        let writable_buffer = if remaining_bytes >= buf.len() {
            &mut buf[..]
        } else {
            &mut buf[..remaining_bytes]
        };
        self.reader.read(writable_buffer)
    }
}

impl<T: BufRead> BufRead for FilePortion<T> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        let remaining_bytes = self.remaining_bytes() as usize;
        let inner_buf = self.reader.fill_buf()?;
        Ok(if inner_buf.len() < remaining_bytes {
            &inner_buf[..remaining_bytes]
        } else {
            inner_buf
        })
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

#[test]
fn no_decimal() {
    assert_eq!(dumb_parse_number("14"), 14.0);
}

#[test]
fn one_decimal() {
    assert_eq!(dumb_parse_number("13.3"), 13.3);
}

#[test]
fn many_decimals() {
    assert_eq!(dumb_parse_number("17.125"), 17.125);
}

#[test]
fn negative() {
    assert_eq!(dumb_parse_number("-9.92"), -9.92);
}
