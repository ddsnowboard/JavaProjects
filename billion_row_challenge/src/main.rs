use billion_row_challenge::write_cities;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let output = File::create("output.txt").unwrap();
    let writer = BufWriter::new(output);
    write_cities(writer);
}
