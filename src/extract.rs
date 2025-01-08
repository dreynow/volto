use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use serde_json::Value;
use crate::dataframe::DataFrame;

#[derive(Debug)]
pub enum FileType {
    Csv,
    Json,
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

    pub fn reader(&self, file_type: FileType) -> io::Result<()> {
        match file_type {
            FileType::Csv => self.read_csv(),
            FileType::Json => self.read_json(),
            FileType::Binary => self.read_binary(),
        }
    }

    pub fn read_csv(&self) -> io::Result<DataFrame> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut dataframe = DataFrame::new();
        let mut headers: Vec<String> = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            let fields: Vec<&str> = line.split(',').map(|s| s.trim().to_string()).collect();

            if index == 0 {
                headers = fields;
                for header in &headers {
                    dataframe.add_column(header.clone(), Vec::new());
                }
            }else {
                for (i, field) in fields.iter().enumerate() {
                    if let Some(header) = headers.get(i) {
                        if let Some(column) = dataframe.columns.get_mut(header) {
                            column.push(field.clone());
                        }
                    }
                }
            }
            
        }

        Ok((dataframe))
    }

    pub fn read_json(&self) -> io::Result<()> {
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json: Value = match serde_json::from_str(&content) {
            Ok(parsed) => parsed,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };
        println!("Parsed JSON: {:?}", json);

        Ok(())
    }

    pub fn read_binary(&self) -> io::Result<()> {
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;
        println!("Read {} bytes from the binary file.", buffer.len());

        Ok(())
    }
}
