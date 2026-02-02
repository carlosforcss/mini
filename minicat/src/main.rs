use std::fs;
use std::env;

fn get_file_content(file_name: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_name)
}

fn main() {
    let filename = env::args().nth(1).expect("Please provilde a file name as an argument");
    let content = get_file_content(&filename).expect("Could not read the file"); 
    print!("{}", content);
}
