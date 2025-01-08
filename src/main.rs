mod extract;
mod transform;
mod load;

use crate::extract::{FileReader, FileType};

fn main() -> std::io::Result<()> {
    let file_reader = FileReader::new("sample.csv");

    if let Err(e) = file_reader.reader(FileType::Csv) {
        eprintln!("Error reading file: {}", e);
    }

    Ok(())
}
