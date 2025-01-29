#[cfg(test)]
mod tests {
    use super::*;
    use crate::deepseek::DeepSeekAI;
    use crate::cache::TransformationCache;
    use crate::transform::Transformer;

    #[test]
    fn test_ai_transformation_with_approval() {
        let transformer = Transformer::new("test_api_key", false);

        let input_data = "Hello, world!";
        let transformation_type = "uppercase";

        // Simulate AI response
        let ai_response = "HELLO, WORLD!".to_string();
        transformer.cache.set(&format!("{}:{}", transformation_type, input_data), &ai_response);

        // Run transformation
        let result = transformer.transform(input_data, transformation_type);

        assert_eq!(result, "HELLO, WORLD!", "AI transformation did not apply correctly");
    }

    #[test]
    fn test_ai_transformation_rejects() {
        let transformer = Transformer::new("test_api_key", false);

        let input_data = "Hello, world!";
        let transformation_type = "uppercase";

        // Simulate user rejection (mock user input)
        let result = transformer.transform(input_data, transformation_type);

        assert_eq!(result, input_data, "Transformation should return original data when rejected");
    }

    #[test]
    fn test_cache_usage() {
        let transformer = Transformer::new("test_api_key", false);
        let input_data = "cached text";
        let transformation_type = "reverse";

        // Manually cache a transformation
        transformer.cache.set(&format!("{}:{}", transformation_type, input_data), "txet dehcac");

        let result = transformer.transform(input_data, transformation_type);
        assert_eq!(result, "txet dehcac", "Cached result was not used correctly");
    }
}
