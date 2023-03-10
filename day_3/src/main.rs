use std::io::{self, BufReader, BufRead, Read};
use std::fs::File;
use std::path::Path;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    println!("Day 03");

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

    let mut total_priority = 0;

    // Loop through each line of the input
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Unable to read lines")).collect();
    for line in &lines {
        let (first, second) = line.split_at(line.len()/2);
        let first_chars: Vec<char> = first.chars().collect();
        let second_chars: Vec<char> = second.chars().collect();

        let common = find_common_char(first_chars, second_chars);

        let cur_value = char_value(common);
        if cur_value > 0 {
            total_priority += cur_value;
        } else {
            panic!("Invalid char found! {common}");
        }
    }

    println!("Total priority: {total_priority}");

    // Part 2
    println!("Part 2");

    let mut total_priority = 0;
    for group in lines.chunks(3) {
        let first: Vec<char> = group[0].chars().collect();
        let second: Vec<char> = group[1].chars().collect();
        let third: Vec<char> = group[2].chars().collect();

        let common = find_common_char_threeway(first, second, third);

        if common != ' ' {
            let cur_value = char_value(common);
            if cur_value > 0 {
                total_priority += cur_value;
            } else {
                panic!("Invalid char found! {common}");
            }
        } else {
            panic!("No common char found");
        }
    }

    println!("Total priority part 2: {total_priority}");

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

fn find_common_char(first: Vec<char>, second: Vec<char>) -> char {
    for ch in first {
        if second.contains(&ch) {
            return ch;
        }
    }
    ' '
}

fn find_common_char_threeway(first: Vec<char>, second: Vec<char>, third: Vec<char>) -> char {
    for ch in first {
        if second.contains(&ch) {
            if third.contains(&ch) {
                return ch;
            }
        }
    }
    ' '
}

fn char_value(c: char) -> i32 {
    let ascii = c as u32;

    if (ascii >= 'A' as u32) && (ascii <= 'Z' as u32) {
        return (ascii - ('A' as u32) + 27) as i32;
    } else if (ascii >= 'a' as u32) && (ascii <= 'z' as u32) {
        return (ascii - ('a' as u32) + 1) as i32;
    }

    -1
}
