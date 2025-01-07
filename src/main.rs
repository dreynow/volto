use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use serde_json::Value;
use quick_xml::Reader as XmlReader;


fn main() -> io::Result<()> {

    let file_path = "sample.csv";

    reader_file(file_path)?;

    Ok(())


}

// read_txt
enum FileType { 
    Csv,
    Json,
    Xml,
    Binary,
}

pub struct FileReader {
    file_path: String,
}

impl FileReader {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn reader(&self, file_type:FileType) -> io::Result<()> {
        match file_type {
            FileType::Csv => self.read_csv(),
            FileType::Json => self.read_json(),
            FileType::Xml => self.read_xml(),
            FileType::Binary => self.read_binary(),
        }
        Ok(())
    }

    /// Read CSV files
    pub fn read_csv(&self) -> io::Result<()> {
        let file = File::open(file)?;
        let reader = io::BufReader::new(file);

        for (_index, line) in reader.lines().enumerate() {
            let line = line?;

            let fields: Vec<&str> = line.split(',').collect();
            println!("{:?}", fields);
        }

        Ok(())
    }

    fn read_json(&self) -> io::Result<()> {
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json: Value = serde_json::from_str(&content).expect("Invalid JSON format");
        println!("Parsed JSON {:?}", json);

        Ok(())
    }

    fn read_xml(&self) -> io::Result<()> {
        let mut file = File::open(&self.file_path);
        let mut reader = BufReader::new(file);

        let mut xml_reader = XmlReader::from_reader(reader);
        xml_reader.trim_text(true);

        for event in xml_reader.into_inter() {
            match event {
                Ok(quick_xml::events::Event::Start(e)) => println!("XML Start tag: {:?}", e.name()),
                Ok(quick_xml::events::Event::Text(e)) => {
                    println!("XML Text: {:?}", e.unescape_and_decode(&xml_reader).unwrap())
                }
                Ok(quick_xml::events::Event::End(e)) => println!("XML End tag: {:?}", e.name()),
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)), // Handle XML parsing errors
                _ => (), // Ignore other events
            }
        }

        Ok(())
    
    }

        /// Method to read binary files
    pub fn read_binary(&self) -> io::Result<()> {
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
    
        reader.read_to_end(&mut buffer)?;
        println!("Read {} bytes from the binary file.", buffer.len());
    
        Ok(())
        
    }    

    
}














