use std::fs;

fn main() {
    let data  = fs::read_to_string("data.txt");

    match data {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}