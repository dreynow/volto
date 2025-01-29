use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

pub struct DeepSeekAI {
    api_key: String,
    client: Client,
}

impl DeepSeekAI {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    pub fn query(&self, input_data: &str, transformation_type: &str) -> Result<Value, Box<dyn Error>> {
        let request_body = json!({
            "prompt": format!("Transform this data using {}: {}", transformation_type, input_data),
            "max_tokens": 100,
        });

        let response = self.client.post("https://api.deepseek.com/v1/transform")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()?
            .json::<Value>()?;

        Ok(response)
    }
}
