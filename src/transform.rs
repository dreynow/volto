use crate::deepseek::DeepSeekAI;
use crate::cache::TransformationCache;
use std::io;

pub struct Transformer {
    deepseek: DeepSeekAI,
    cache: TransformationCache,
    debug_mode: bool,
}

impl Transformer {
    pub fn new(api_key: &str, debug_mode: bool) -> Self {
        Self {
            deepseek: DeepSeekAI::new(api_key),
            cache: TransformationCache::new(),
            debug_mode,
        }
    }

    pub fn transform(&self, input: &str, transformation_type: &str) -> String {
        let cache_key = format!("{}:{}", transformation_type, input);

        // Check cache first
        if let Some(cached_result) = self.cache.get(&cache_key) {
            println!("Using cached transformation");
            return cached_result;
        }

        // Query AI for transformation
        let ai_suggestion = match self.deepseek.query(input, transformation_type) {
            Ok(response) => response["transformed_text"].as_str().unwrap_or("Error").to_string(),
            Err(_) => "AI transformation failed".to_string(),
        };

        // Debug mode: Show AI suggestion before applying
        if self.debug_mode {
            println!("AI Suggestion: {}", ai_suggestion);
        }

        // Ask user for approval
        println!("Apply this transformation? (yes/no):");
        let mut approval = String::new();
        io::stdin().read_line(&mut approval).expect("Failed to read input");

        if approval.trim().to_lowercase() == "yes" {
            self.cache.set(&cache_key, &ai_suggestion);
            ai_suggestion
        } else {
            println!("Transformation rejected.");
            input.to_string()
        }
    }
}
