use std::fs;
use std::env;

fn main() {
    let filename = match env::args().nth(1) {
        Some(name) => name,
        None => {
            eprintln!("Please provide a filename as an argument");
            return;
        }
    };
    match fs::read_to_string(&filename) {
        Ok(content) => print!("{}", content),
        Err(_) => {
            eprintln!("Error: cat: {}: No such file or direectory", filename);
            return;
        }
    }; 
}
