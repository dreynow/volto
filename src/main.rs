mod extract;
mod transform;
mod load;
mod dataframe;

use extract::FileReader;


fn main() -> std::io::Result<()> {
    let file_reader = FileReader::new("sample.csv");

    match file_reader.read_csv() {
        Ok(dataframe) => dataframe.display(),
        Err(e) => eprintln!("Error reading CSV file: {}", e),
    }

    Ok(())
}
