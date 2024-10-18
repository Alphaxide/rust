use clap::{Arg, Command};
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let matches = Command::new("File Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Reads from and writes to a file")
        .arg(
            Arg::new("read")
                .short('r')
                .long("read")
                .value_name("FILE")
                .help("Reads content from a file")
                .required(false) // Make this argument optional
                .value_hint(clap::ValueHint::FilePath), // Suggests that the argument is a file path
        )
        .arg(
            Arg::new("write")
                .short('w')
                .long("write")
                .value_name("FILE")
                .help("Writes content to a file")
                .required(false) // Make this argument optional
                .value_hint(clap::ValueHint::FilePath), // Suggests that the argument is a file path
        )
        .arg(
            Arg::new("content")
                .short('c')
                .long("content")
                .value_name("TEXT")
                .help("Content to write into the file")
                .required(false) // Make this argument optional
        )
        .get_matches();

    // Call the appropriate function based on the arguments
    if let Some(filename) = matches.get_one::<String>("read") { // Updated from `value_of` to `get_one`
        match read_file(filename) {
            Ok(content) => println!("File content:\n{}", content),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    } else if let Some(filename) = matches.get_one::<String>("write") { // Updated from `value_of` to `get_one`
        if let Some(content) = matches.get_one::<String>("content") { // Updated from `value_of` to `get_one`
            match write_file(filename, content) {
                Ok(_) => println!("Successfully wrote to the file."),
                Err(e) => eprintln!("Error writing to file: {}", e),
            }
        } else {
            eprintln!("No content provided to write to the file.");
            process::exit(1);
        }
    } else {
        eprintln!("Please specify either --read or --write.");
        process::exit(1);
    }
}

// Function to read a file
fn read_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

// Function to write to a file
fn write_file(filename: &str, content: &str) -> Result<(), io::Error> {
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())
}
