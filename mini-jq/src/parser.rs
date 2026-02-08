use std::fs;
use crate::types::{JSON, JSONValue}; 


pub struct JSONParser{
}

impl JSONParser {

    pub fn new() -> Self {
        JSONParser {}
    }

    pub fn parse(&self, input: &str) -> Result<JSONValue, String> {
        Ok(JSONValue::String("Parsed JSON succesfully".to_string()))
    }

    pub fn from_file(&self, file_path: &str) -> Result<JSONValue, String> {
        let file_content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        self.parse(&file_content)
    }
}


