use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use serde_json::Value;
use crate::dataframe::{self, DataFrame};
use crate::errors::FileReaderError;

#[derive(Debug)]
pub enum FileType {
    Csv,
    Json,
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

    pub fn reader(&self, file_type: FileType) -> io::Result<(DataFrame)> {
        match file_type {
            FileType::Csv => self.read_csv(),
            FileType::Json => self.read_json(),
        }
    }

    pub fn read_csv(&self) -> io::Result<DataFrame> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut dataframe = DataFrame::new();
        let mut headers: Vec<String> = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line.map_err(FileReaderError::IoError)?;
            let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();

            if index == 0 {
                headers = fields;
                for header in &headers {
                    dataframe.add_column(&header.clone(), Vec::new());
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

    pub fn read_json(&self) -> io::Result<(DataFrame)> {
        let mut file = File::open(&self.file_path).map_err(FileReaderError::IoError)?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(FileReaderError::IoError)?;

        let json: Value = serde_json::from_str(&content)
            .map_err(|e| FileReaderError::JsonParseError(e.to_string()))?;

        let mut dataframe = DataFrame::new();

        match json {
            // Case 1: Array of objects
            Value::Array(rows) => {
                if rows.is_empty() {
                    return Err(FileReaderError::EmptyJsonArray);
                }

                if let Some(Value::Object(first_row)) = rows.get(0) {
                    for key in first_row.keys() {
                        dataframe.add_column(key, Vec::new());
                    }
                } else {
                    return Err(FileReaderError::InvalidJsonStructure("JSON array does not contain objects".to_string()));
                }
                

                // Populates rows
                for row in rows {
                    if let Value::Object(row_map) = row {
                        for (key, value) in row_map {
                            if let Some(column) = dataframe.columns.get_mut(&key) {
                                column.push(value_to_string(&value));
                            }
                        }
                    }
                }
            }

            // Case 2: Object with arrays
            Value::Object(columns) => {
                for (key,value) in &columns {
                    if let Value::Array(values) = value {
                        dataframe.add_column(key, values.iter().map(value_to_string).collect());
                    } else {
                        return Err(FileReaderError::InvalidJsonStructure(format!("Value for '{}' is not an array.", key)));
                    }
                }


                // validate array lengths
                let lengths: Vec<usize> = dataframe.columns.values().map(|col| col.len()).collect();
                if lengths.iter().any(|&len| len != lengths[0]) {
                    return Err(FileReaderError::InconsistentArrayLengths);
                }
            }

            // Unsupported structure
            _ => {
                return Err(FileReaderError::UnsupportedJsonStructure);
            }
        }

        

        fn value_to_string(value: &Value) -> String {
            match value {
                Value::Null => "null".to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Number(n) => n.to_string(),
                Value::String(s) => s.clone(),
                _=> format!("{:?}",value),
            }
        }

        Ok(dataframe)
    }


}
