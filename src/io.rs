use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_maps() -> Result<Vec<String>> {
    let mut file = File::open("data/maps.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if line.is_empty() {
            continue;
        }
        // TODO: handle the line
    }
}
