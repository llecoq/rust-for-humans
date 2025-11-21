use std::fs::File;
use std::io::{self, Read};

// A function that returns an Option
// It returns Some(length) if the text is not empty
// Otherwise it returns None
fn get_length_if_not_empty(text: &str) -> Option<usize> {
    if text.is_empty() {
        None
    } else {
        Some(text.len())
    }
}

// A function that returns a Result,
// because reading a file can fail
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; 
    // `?` means:
    // if File::open returns Err, return it immediately

    let mut contents = String::new();
    file.read_to_string(&mut contents)?; 
    // Same logic: propagate error if any

    Ok(contents) // Return the contents if everything went well
}

fn main() {
    // ----------------------------------------
    // 1. Using Option
    // ----------------------------------------
    let text = "hello";

    match get_length_if_not_empty(text) {
        Some(len) => println!("Length is {}", len),
        None => println!("The string was empty"),
    }

    // ----------------------------------------
    // 2. Using Result and ?
    // ----------------------------------------
    match read_file_contents("example.txt") {
        Ok(data) => println!("File content:\n{}", data),
        Err(e) => println!("Could not read file: {}", e),
    }
}
