use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env::args;

#[derive(Debug)]
struct instruction {
    count: u32,
    from: u32,
    to: u32,
}

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

    // Need the number of stacks
    let stack_labels = lines[line_break-1].split(' ').collect::<Vec<&str>>();
    let num_stacks = stack_labels[stack_labels.len() - 2].parse::<u32>().unwrap();

    // Build the stack vector
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for i in 0..num_stacks {
        let tmp_vec: Vec<char> = Vec::new();
        stacks.push(tmp_vec);
    }

    // Put together the stacks
    for i in (0..line_break-1).rev() {
        let line_array: Vec<char> = lines[i].chars().collect();
        for j in (1..line_array.len() - 1).step_by(4) {
            if line_array[j] != ' ' {
                stacks[(j-1)/4].push(line_array[j]);
            }
        }
    }

    // Clone for part 2
    let mut stacks_p2: Vec<Vec<char>> = stacks.clone();

    // Grab instructions
    let mut instruction_list: Vec<instruction> = Vec::new();
    for i in line_break+1..lines.len() {
        let cur_line: Vec<&str> = lines[i].split(' ').collect();
        let cur_instruction: instruction = instruction { 
            count: (cur_line[1].parse().unwrap()), 
            from: (cur_line[3].parse().unwrap()), 
            to: (cur_line[5].parse().unwrap()),
        };
        instruction_list.push(cur_instruction);
    }

    // Finally, perform the instructions
    for instruction in &instruction_list {
        perform_instruction(&instruction, &mut stacks);
    }

    print!("Top of stacks: ");
    for i in 0..stacks.len() {
        print!("[{}] ", stacks[i].last().unwrap());
    }

    // Part 2
    println!("\nPart 2!");

    // Perform the instructions for part 2
    for instruction in &instruction_list {
        perform_instruction_p2(&instruction, &mut stacks_p2);
    }

    print!("Top of stacks part 2: ");
    for i in 0..stacks_p2.len() {
        print!("[{}] ", stacks_p2[i].last().unwrap());
    }
}

fn perform_instruction(instruction: &instruction, stacks: &mut Vec<Vec<char>>) -> bool {
    let ret = true;

    let count = instruction.count;
    let from = instruction.from;
    let to = instruction.to;

    let mut tmp: char;
    for i in 0..count {
        tmp = stacks[(from-1) as usize].pop().unwrap();
        stacks[(to-1) as usize].push(tmp);
    }

    return ret;
}

fn perform_instruction_p2(instruction: &instruction, stacks: &mut Vec<Vec<char>>) -> bool {
    let ret = true;

    let count = instruction.count;
    let from = instruction.from;
    let to = instruction.to;

    let mut tmp: Vec<char> = Vec::new();
    for i in 0..count {
        tmp.push(stacks[(from-1) as usize].pop().unwrap());
    }
    for i in 0..count {
        stacks[(to-1) as usize].push(tmp.pop().unwrap());
    }

    return ret;
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
