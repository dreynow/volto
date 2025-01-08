use std::fs::File;
use std::collections::HashMap;

pub struct Dataframe {
    columns: HashMap<String, Vec<String>>,
}

impl Dataframe {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }        
    }

    pub fn add_column(&mut self, column_name: &str, data: Vec<String>) {
        self.columns.insert(column_name.to_string(), data);
    }

    pub fn select_column(&self, column_name: &str) -> Option<&Vec<String>> {
        self.column.get(column_name)
    }
}
