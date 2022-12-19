
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
fn main() {
    println!("Let's solve day 1 of the Advent of Code!");

    let file = get_input_file();

    let reader = BufReader::new(file);
    let mut cal_total : Vec<u32> = Vec::new();
    let mut cur_elf_total: u32 = 0;
    for line in reader.lines(){
        let cur_cal = line.expect("Unable to read line");
        if !cur_cal.trim().is_empty(){
            cur_elf_total += cur_cal.trim().parse::<u32>().expect("Unable to parse int");
        }else{
            cal_total.push(cur_elf_total);
            cur_elf_total = 0;
        }
    }

    // PART 1
    println!("Part 1!");

    let mut elf_most_cal_ind: usize = 0;
    let mut most_cal: u32 = 0;
    for (ind, elf) in cal_total.iter().enumerate(){
        if *elf > most_cal{
            most_cal = *elf;
            elf_most_cal_ind = ind;
        }
    }

    println!("Elf {} has the most calories at {most_cal}", elf_most_cal_ind+1);

    // PART 2
    println!("Part 2!");

    // Sort the vector (reverse order)
    cal_total.sort();
    cal_total.reverse();
    let mut top_3_cal = 0;
    for i in 0..3{
        println!("Top {} elf has {} calories", i+1, cal_total[i]);
        top_3_cal += cal_total[i];
    }
    println!("The total for the top 3 is {top_3_cal}");
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
    let path = Path::new(filename);

    // Open the file in read only mode
    println!("Opening file {}", path.display());
    let file = match File::open(&path){
        Err(why) => panic!("Couldn't open {} {}", path.display(), why),
        Ok(file) => file,
    };
    file
}
