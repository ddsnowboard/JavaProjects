use fxhash::FxHashMap;
use memchr::memchr;
use memmap::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

const FILENAME: &str = "measurements.txt";

type Temp = f64;

pub struct Row {
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

fn get_cities(file: &[u8]) -> Vec<(String, City)> {
    let mut cities = FxHashMap::default();
    let mut current_index = 0;
    while current_index < file.len() {
        let next_line = {
            match memchr(b'\n', &file[current_index..]) {
                Some(newline_idx) => &file[current_index..(current_index + newline_idx)],
                _ => &file[current_index..],
            }
        };
        current_index += next_line.len() + 1;
        let row = read_row(std::str::from_utf8(next_line).unwrap());
        let city = cities.entry(row.city).or_insert_with(City::new);
        city.process_temp(row.temp);
    }
    let mut cities: Vec<_> = cities.into_iter().collect();
    cities.sort_unstable_by_key(|(c, _)| c.clone());
    cities
}

fn split_up_file(file: &[u8], n_threads: usize) -> Vec<&[u8]> {
    let naive_block_size = file.len() / n_threads;
    let naive_block_starts = (0..n_threads).map(|n| n * naive_block_size);
    let actual_block_starts: Vec<_> = naive_block_starts
        .map(|start| {
            let mut start = start;
            if start == file.len() - 1 {
                panic!("This should never happen!")
            } else {
                while start > 0 && file[start - 1] != b'\n' {
                    start -= 1
                }
                Some(start)
            }
        })
        // This is so that the last window ends in None
        .chain(std::iter::once(None))
        .collect();

    let start_ends = actual_block_starts.windows(2);
    start_ends
        .map(|slice| match *slice {
            [Some(start), Some(end)] => &file[start..end],
            [Some(start), None] => &file[start..],
            _ => panic!("Hmm how'd this happen?"),
        })
        .collect()
}

fn merge_maps(
    mut m1: FxHashMap<String, City>,
    m2: FxHashMap<String, City>,
) -> FxHashMap<String, City> {
    m2.into_iter().for_each(|(name, city)| {
        let value = m1.entry(name).or_insert_with(City::new);
        value.temps.extend(city.temps);
    });
    m1
}

pub fn write_cities<W: std::io::Write>(mut writer: BufWriter<W>) {
    // 8 threads is best for the desktop, 2 for the laptop
    let n_threads = 2;
    let file = get_file();
    let file_handles: Vec<&[u8]> = split_up_file(&file, n_threads);
    let mut cities: Vec<(String, City)> = file_handles
        .into_par_iter()
        .flat_map(get_cities)
        .fold(FxHashMap::default, |mut hm, (name, city)| {
            let val = hm.entry(name).or_insert_with(City::new);
            val.temps.extend(city.temps);
            hm
        })
        .reduce_with(merge_maps)
        .unwrap()
        .into_iter()
        .collect();
    cities.par_sort_unstable_by(|(name1, _), (name2, _)| name1.cmp(name2));

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
// Also I could just split the whole file into pieces and then give each piece to a thread.
// Got options.
pub fn read_row(row: &str) -> Row {
    let semicolon_idx = match memchr(b';', row.as_bytes()) {
        Some(idx) => idx,
        None => panic!("There was no semicolon; row was \"{}\"", row),
    };
    let city = String::from(&row[..semicolon_idx]);
    let temp = &row[(semicolon_idx + 1)..];
    Row {
        city,
        temp: dumb_parse_number(temp),
    }
}

fn get_file() -> Mmap {
    unsafe { memmap::Mmap::map(&File::open(FILENAME).unwrap()).unwrap() }
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

#[cfg(test)]
mod reader_tests {
    use crate::split_up_file;
    const LINE_LENGTH: usize = 10;
    fn generate_test_file(n_lines: u64) -> String {
        let mut out = String::new();
        for idx in 0..n_lines {
            out.push_str(&format!("{}\n", format_number(idx)));
        }
        out
    }
    fn format_number(n: u64) -> String {
        format!("{:09}", n)
    }

    #[test]
    fn test_split_up_file() {
        let n_lines = 5000;
        let file = generate_test_file(n_lines);
        let n_chunks = 7;
        let actual_chunks: Vec<&[u8]> =
            split_up_file(file.as_bytes(), n_chunks.try_into().unwrap());
        assert_eq!(actual_chunks.len(), n_chunks);
        let lengths: Vec<_> = actual_chunks.iter().map(|v| v.len()).collect();
        assert_eq!(lengths.iter().sum::<usize>(), file.len());
        let longest_length = *lengths.iter().max().unwrap();
        let shortest_length = *lengths.iter().min().unwrap();
        assert!(
            (longest_length - shortest_length) * 10 < longest_length,
            "The lengths are too different"
        );

        let recombined: Vec<u8> = actual_chunks.into_iter().flatten().copied().collect();
        assert_eq!(&recombined, file.as_bytes());
    }
}