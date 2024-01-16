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

fn get_cities(mut reader: impl BufRead) -> Vec<(String, City)> {
    let mut cities = FxHashMap::default();
    let mut row_holder = String::with_capacity(128);
    loop {
        row_holder.clear();
        match reader.read_line(&mut row_holder).unwrap() {
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
    let min_line_index = chunk_index * expected_lines_per_chunk;
    let last_line_index = min_line_index + lines_for_this_chunk;
    let (start_offset, end_offset) = {
        file.rewind().unwrap();
        let file = file.try_clone().unwrap();
        let offsets: Vec<(u64, u64)> = line_offsets(file)
            .skip(min_line_index as usize)
            .take((last_line_index - min_line_index) as usize)
            .collect();
        (offsets[0].0, offsets.last().unwrap().1)
    };
    file.seek(SeekFrom::Start(start_offset)).unwrap();
    FilePortion::new(BufReader::new(file), end_offset - start_offset)
}

fn line_offsets(file: File) -> Box<dyn Iterator<Item = (u64, u64)>> {
    let mut reader = BufReader::new(file);
    let mut next_start_offset: u64 = 0;
    let mut buf: Vec<u8> = Vec::new();
    Box::new(std::iter::from_fn(move || {
        let start = next_start_offset;
        buf.clear();
        reader.read_until(b'\n', &mut buf).unwrap();
        if *buf.last().unwrap() == b'\n' {
            let end_offset = start + (buf.len() as u64);
            next_start_offset = end_offset;
            Some((start, end_offset))
        } else {
            None
        }
    }))
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
    let file_handles = (0..n_threads)
        .collect::<Vec<u32>>()
        .into_par_iter()
        .map(|idx| split_up_file(get_file(), idx, n_threads));
    let mut cities: Vec<(String, City)> = file_handles
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
        self.max_offset_exclusive
            .checked_sub(self.current_offset)
            .and_then(|r| r.checked_sub(1))
            .unwrap_or(0)
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
        Ok(if inner_buf.len() > remaining_bytes {
            &inner_buf[..remaining_bytes]
        } else {
            inner_buf
        })
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt);
        self.current_offset += amt as u64;
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

#[cfg(test)]
mod reader_tests {
    use crate::line_offsets;
    use crate::split_up_file;
    use crate::FilePortion;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::io::Write;
    use tempfile::tempfile;
    const LINE_LENGTH: u64 = 10;
    fn generate_test_file(n_lines: u64) -> File {
        let mut file = tempfile().unwrap();
        for idx in 0..n_lines {
            write!(file, "{}\n", format_number(idx)).unwrap();
        }
        file.rewind().unwrap();
        file
    }
    fn format_number(n: u64) -> String {
        format!("{:09}", n)
    }

    #[test]
    fn read_big_file() {
        let n_lines = 10_000;
        let start_line: u64 = 440;
        let length_lines = 2000;
        let mut file = generate_test_file(n_lines);
        file.seek(SeekFrom::Start((start_line * LINE_LENGTH).into()))
            .unwrap();
        let reader = BufReader::new(file);
        let reader = FilePortion::new(reader, length_lines * LINE_LENGTH);
        let expected: Vec<String> = (start_line..(start_line + length_lines))
            .map(format_number)
            .collect();
        let actual: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split_up_file() {
        let n_lines = 5000;
        let file = generate_test_file(n_lines);
        let n_chunks = 7;
        let this_chunk = 4;
        let expected_chunk_size = n_lines / n_chunks;
        let reader = split_up_file(
            file,
            this_chunk.try_into().unwrap(),
            n_chunks.try_into().unwrap(),
        );
        let expected_lines: Vec<String> = ((this_chunk * expected_chunk_size)
            ..((this_chunk + 1) * expected_chunk_size))
            .map(format_number)
            .collect();
        let actual_lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        assert_eq!(actual_lines, expected_lines);
    }

    #[test]
    fn test_get_line_start_indexes() {
        let n_lines = 100;
        let file = generate_test_file(n_lines);
        let expected_line_offsets: Vec<_> = (0..n_lines)
            .map(|l| (LINE_LENGTH * l, LINE_LENGTH * (l + 1) - 1))
            .collect();
        let actual_line_offsets: Vec<_> = line_offsets(file).collect();
        assert_eq!(n_lines as usize, actual_line_offsets.len());
        assert_eq!(expected_line_offsets, actual_line_offsets);
    }
}
