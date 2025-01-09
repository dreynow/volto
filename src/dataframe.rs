use std::collections::HashMap;

pub struct DataFrame {
    pub columns: HashMap<String, Vec<String>>,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }        
    }

    pub fn add_column(&mut self, column_name: &str, data: Vec<String>) {
        self.columns.insert(column_name.to_string(), data);
    }

    pub fn select_column(&self, column_name: &str) -> Option<&Vec<String>> {
        self.columns.get(column_name)
    }

    pub fn display(&self) {
        for (name,values) in &self.columns {
            println!("Column: {}", name);
            println!("Values: {:?}", values);
        }
    }
}
