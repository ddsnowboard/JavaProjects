use fxhash::FxHashMap;
use memchr::memchr;
use memmap::Mmap;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

const FILENAME: &str = "measurements.txt";

type Temp = f64;

pub struct Row<'a> {
    city: &'a str,
    temp: Temp,
}

struct City {
    count: usize,
    current_average: Option<Temp>,
    current_min: Option<Temp>,
    current_max: Option<Temp>,
}

fn max(a: Temp, b: Temp) -> Temp {
    match a.partial_cmp(&b).unwrap() {
        Ordering::Less => b,
        _ => a,
    }
}

fn min(a: Temp, b: Temp) -> Temp {
    match a.partial_cmp(&b).unwrap() {
        Ordering::Less => a,
        _ => b,
    }
}

impl City {
    fn new() -> Self {
        City {
            count: 0,
            current_max: None,
            current_min: None,
            current_average: None,
        }
    }
    fn process_temp(&mut self, temp: Temp) {
        self.current_max = Some(match self.current_max {
            Some(m) => max(m, temp),
            None => temp,
        });
        self.current_min = Some(match self.current_min {
            Some(m) => min(m, temp),
            None => temp,
        });
        self.current_average = Some(match self.current_average {
            Some(a) => ((a * (self.count as Temp)) + temp) / ((self.count + 1) as Temp),
            None => temp,
        });
        self.count += 1;
    }

    fn get_min(&self) -> Temp {
        self.current_min.unwrap()
    }

    fn get_max(&self) -> Temp {
        self.current_max.unwrap()
    }

    fn get_mean(&self) -> Temp {
        self.current_average.unwrap()
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

    fn combine(&mut self, other: &Self) {
        if self.count != 0 {
            let total_count = self.count + other.count;
            self.current_max = Some(max(self.get_max(), other.get_max()));
            self.current_min = Some(min(self.get_min(), other.get_min()));
            self.current_average = Some(
                (((self.count as Temp) * self.get_mean())
                    + ((other.count as Temp) * other.get_mean()))
                    / (total_count as Temp),
            );
            self.count = total_count;
        } else {
            self.count = other.count;
            self.current_min = other.current_min;
            self.current_max = other.current_max;
            self.current_average = other.current_average;
        }
    }
}

fn get_cities(file: &[u8]) -> Vec<(&str, City)> {
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
        let row = read_row(unsafe { std::str::from_utf8_unchecked(next_line) });
        let city = cities.entry(row.city).or_insert_with(City::new);
        city.process_temp(row.temp);
    }
    cities.into_iter().collect()
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

fn merge_maps<'a>(
    mut m1: FxHashMap<&'a str, City>,
    m2: FxHashMap<&'a str, City>,
) -> FxHashMap<&'a str, City> {
    m2.into_iter().for_each(|(name, city)| {
        let value = m1.entry(name).or_insert_with(City::new);
        value.combine(&city);
    });
    m1
}

pub fn write_cities<W: std::io::Write>(mut writer: BufWriter<W>) {
    // 16 threads is best for the desktop, 2 for the laptop
    let n_threads = 16;
    let file = get_file();
    let file_handles: Vec<&[u8]> = split_up_file(&file, n_threads);
    let mut cities: Vec<(&str, City)> = file_handles
        .into_par_iter()
        .flat_map(get_cities)
        .fold(FxHashMap::default, |mut hm, (name, city)| {
            let val = hm.entry(name).or_insert_with(City::new);
            val.combine(&city);
            hm
        })
        .reduce_with(merge_maps)
        .unwrap()
        .into_iter()
        .collect();
    cities.par_sort_unstable_by(|(name1, _), (name2, _)| name1.cmp(name2));

    let strings: Vec<_> = cities
        .into_iter()
        .map(|(name, city)| city.to_str(name))
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
    let city = &row[..semicolon_idx];
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
