use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {

    let file_path = "sample.csv";

    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        let fields: Vec<&str> = line.split(',').collect();
        println!("Row {}: {:?}",index, fields);
    }

    Ok(())



}

