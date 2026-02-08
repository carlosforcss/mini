mod types;
mod parser;

use crate::parser::JSONParser;

fn main() {
    let parser = JSONParser::new();
    let content = parser.from_file("static/test_file.json").expect("Failed to parse JSON file");
    println!("Parsed content: {:?}", content);
}

