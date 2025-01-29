use std::collections::HashMap;
use std::sync::Mutex;

pub struct TransformationCache {
    cache: Mutex<HashMap<String, String>>,
}

impl TransformationCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.lock().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: &str, value: &str) {
        self.cache.lock().unwrap().insert(key.to_string(), value.to_string());
    }
}
