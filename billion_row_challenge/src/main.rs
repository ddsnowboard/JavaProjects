use std::fs::File;
use std::io::BufWriter;
use billion_row_challenge::write_cities;

fn main() {
    let output = File::create("/dev/null").unwrap();
    let writer = BufWriter::new(output);
    write_cities(writer);
}

