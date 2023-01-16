use std::io::{self, BufReader, BufRead};
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
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Unable to read lines")).collect();
    let line_break = lines.iter().position(|x| x.eq("")).expect("Unable to separate stacks from instructions");

    let num_stacks = lines[line_break-1].split(' ').collect::<Vec<&str>>(); // Need the number of stacks
    for i in 0..line_break-1 {
        println!("Line {}: {:?}", i, lines[i]);
    }
    println!("Number of stacks: {:?}", num_stacks);
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
