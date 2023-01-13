use std::io::{self, BufReader, BufRead, Read};
use std::fs::File;
use std::path::Path;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    println!("Day 05");

    let file: File;
    if &args.len() > &1 {
        if Path::new(&args[1]).exists() {
            file = open_file_with_handling(&args[1]);
        } else {
            println!("Provided file not found, try again");
            file = get_input_file();
        }
    } else {
        // Get input file
        file = get_input_file();
    }

    // Loop through each line of the input
    let mut elf_pairs: Vec<ElfPair> = Vec::new();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Unable to read lines")).collect();
    while line in &lines != "\n" {
        println!("{}", line);
    }


}

fn get_input_file() -> File {
    // Read in input filename from user
    println!("Path to input file: ");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    // Trim off the newline and convert to path
    let filename = filename.trim();
    let file = open_file_with_handling(filename);
    file
}

fn open_file_with_handling(filename: &str) -> File {
    let path = Path::new(filename);

    // Open the file in read only mode
    println!("Opening file {}", path.display());
    let file = match File::open(&path){
        Err(why) => panic!("Couldn't open {} {}", path.display(), why),
        Ok(file) => file,
    };
    file
}
