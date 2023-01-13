use std::io::{self, BufReader, BufRead, Read};
use std::fs::File;
use std::path::Path;
use std::env::args;

enum Elf {
    ELF1(u32, u32),
    ELF2(u32, u32)
}

struct ElfPair {
    elf_1: Elf,
    elf_2: Elf
}

fn main() {
    let args: Vec<String> = args().collect();
    println!("Day 04");

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
    for line in &lines {
        elf_pairs.push(get_ranges(&line));
    }

    let mut count = 0;
    for pair in &elf_pairs {
        if check_if_range_fully_contained(&pair) {
            count += 1;
        }
    }

    println!("Total count: {}", count);

    // Part 2
    println!("Part 2:");

    let mut count_any_overlap = 0;
    for pair in &elf_pairs {
        if check_if_overlapped(&pair) {
            count_any_overlap += 1;
        }
    }

    println!("Total with any overlap: {}", count_any_overlap);
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

fn get_ranges(line: &str) -> ElfPair {
    // Get the two pairs in string form
    let elf_pair: Vec<&str> = line.split(',').collect();
    // Pull out each elf's range
    let elf1: Vec<&str> = elf_pair[0].split('-').collect();
    let elf2: Vec<&str> = elf_pair[1].split('-').collect();
    // Build the struct
    let elf_pair = ElfPair {
        elf_1: Elf::ELF1(elf1[0].parse().expect("Invalid string found"), elf1[1].parse().expect("Invalid string found")),
        elf_2: Elf::ELF2(elf2[0].parse().expect("Invalid string found"), elf2[1].parse().expect("Invalid string found")),
    };
    elf_pair
}

fn check_if_range_fully_contained(pair: &ElfPair) -> bool {
    let mut elf1: Vec<u32> = Vec::new();
    let mut elf2: Vec<u32> = Vec::new();
    if let Elf::ELF1(start, end) = pair.elf_1 {
        elf1.push(start);
        elf1.push(end);
    }
    if let Elf::ELF2(start, end) = pair.elf_2 {
        elf2.push(start);
        elf2.push(end);
    }

    ((elf1[0] <= elf2[0]) && (elf1[1] >= elf2[1])) || ((elf2[0] <= elf1[0]) && (elf2[1] >= elf1[1]))
}

fn check_if_overlapped(pair: &ElfPair) -> bool {
    let mut elf1: Vec<u32> = Vec::new();
    let mut elf2: Vec<u32> = Vec::new();
    if let Elf::ELF1(start, end) = pair.elf_1 {
        elf1.push(start);
        elf1.push(end);
    }
    if let Elf::ELF2(start, end) = pair.elf_2 {
        elf2.push(start);
        elf2.push(end);
    }

    ((elf1[0] <= elf2[1]) && (elf1[1] >= elf2[0])) || ((elf2[0] <= elf1[1]) && (elf2[1] >= elf1[0]))
}
