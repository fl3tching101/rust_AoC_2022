use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::marker;
use std::path::Path;
use std::env::args;
use std::collections::VecDeque;

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
    let mut reader = BufReader::new(&file);
    let mut input = String::new();
    let err = reader.read_line(&mut input);
    if err.is_err() {
        panic!("Unable to read input file");
    }

    // Setup
    let mut queue: VecDeque<char> = VecDeque::with_capacity(4);
    let input_chars = input.chars();
    let mut found = false;
    let mut marker_pos = 0;

    for c in input_chars {
        queue.push_back(c);
        marker_pos += 1;
        if queue.len() >= 4 {
            found = check_for_marker(&queue);
            if found == true {
                break;
            }
            queue.pop_front();
        }
    }

    println!("Marker found at position {}", marker_pos);
}

fn check_for_marker(queue: &VecDeque<char>) -> bool {
    for i in 0..queue.len() {
        for j in 0..queue.len() {
            if i == j {
                continue;
            }
            let outer_char = *queue.get(i).unwrap();
            let inner_char = *queue.get(j).unwrap();
            if outer_char == inner_char {
                return false;
            }
        }
    }
    return true;
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
