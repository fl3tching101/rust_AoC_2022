use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

enum ShapeScore{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum RoundScore{
    Loss = 0,
    Draw = 3,
    Win = 6,
}

const OPPONENT_SHAPE_SCORE: [(char, u32); 3] = [
    ('A', 1),
    ('B', 2),
    ('C', 3),
];

const SELF_SHAPE_SCORE: [(char, u32); 3] = [
    ('X', 1),
    ('Y', 2),
    ('Z', 3),
];

fn main() {
    println!("Day 02");

    // Get input file
    let file = get_input_file();

    // Generate a HashMap to map chars to scores
    let opp_shape_score = HashMap::from(OPPONENT_SHAPE_SCORE);
    let self_shape_score = HashMap::from(SELF_SHAPE_SCORE);

    // Score for the round
    let mut opp_round_score: u32 = 0;
    let mut self_round_score: u32 = 0;

    // Score totals
    let mut opp_total_score: u32 = 0;
    let mut self_total_score: u32 = 0;

    // Loop through each line of the input
    let reader = BufReader::new(file);
    for line in reader.lines(){
        // Convert the line into a vector of chars
        let line = line.expect("Unable to read line");
        let chars: Vec<char> = line.chars().collect();

        // Get opponent score for this round
        opp_round_score = match opp_shape_score.get(&chars[0]) {
            Some(&o) => o,
            None => panic!("Something went wrong with HashMap")
        };
        // Get self score for this round
        self_round_score = match self_shape_score.get(&chars[2]) {
            Some(&o) => o,
            None => panic!("Something went wrong with HashMap")
        };

        // Add the win/loss/draw score for opponent
        opp_total_score += if opp_round_score > self_round_score {
            RoundScore::Win as u32
        } else if opp_round_score < self_round_score {
            RoundScore::Loss as u32
        } else {
            RoundScore::Draw as u32
        };

        // Add the win/loss/draw score for self
        self_total_score += if opp_round_score > self_round_score {
            RoundScore::Loss as u32
        } else if opp_round_score < self_round_score {
            RoundScore::Win as u32
        } else {
            RoundScore::Draw as u32
        };

        // Add the round score
        opp_total_score += opp_round_score;
        self_total_score += self_round_score;

        // Reset round score
        opp_round_score = 0;
        self_round_score = 0;
    }

    println!("Opponent total score: {opp_total_score}");
    println!("Self total score: {self_total_score}");
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
