use std::time::Instant;

use backend::utils::csv_reader::read_files;

fn main() {
    let start = Instant::now();
    // reads stations
    read_files();

    let end = Instant::now();

    println!("Duration: {:?}", (end - start));
}
