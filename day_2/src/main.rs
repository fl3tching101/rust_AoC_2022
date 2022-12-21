use std::io::{self, BufReader, BufRead, Seek};
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

const SELF_STRATEGY_SCORE: [(char, u32); 3] = [
    ('X', 0),
    ('Y', 3),
    ('Z', 6),
];

const RPS_OUTCOME_MAP: [((u32, u32), u32); 9] = [
    ((ShapeScore::Rock as u32, ShapeScore::Rock as u32), RoundScore::Draw as u32),
    ((ShapeScore::Rock as u32, ShapeScore::Paper as u32), RoundScore::Loss as u32),
    ((ShapeScore::Rock as u32, ShapeScore::Scissors as u32), RoundScore::Win as u32),
    ((ShapeScore::Paper as u32, ShapeScore::Rock as u32), RoundScore::Win as u32),
    ((ShapeScore::Paper as u32, ShapeScore::Paper as u32), RoundScore::Draw as u32),
    ((ShapeScore::Paper as u32, ShapeScore::Scissors as u32), RoundScore::Loss as u32),
    ((ShapeScore::Scissors as u32, ShapeScore::Rock as u32), RoundScore::Loss as u32),
    ((ShapeScore::Scissors as u32, ShapeScore::Paper as u32), RoundScore::Win as u32),
    ((ShapeScore::Scissors as u32, ShapeScore::Scissors as u32), RoundScore::Draw as u32),
];

fn main() {
    println!("Day 02");

    // Get input file
    let file = get_input_file();

    // Generate a HashMap to map chars to scores
    let opp_shape_score = HashMap::from(OPPONENT_SHAPE_SCORE);
    let self_shape_score = HashMap::from(SELF_SHAPE_SCORE);
    let self_strategy_score = HashMap::from(SELF_STRATEGY_SCORE);

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
        opp_total_score += determine_rps_win(opp_round_score, self_round_score);

        // Add the win/loss/draw score for self
        self_total_score += determine_rps_win(self_round_score, opp_round_score);

        // Add the round score
        opp_total_score += opp_round_score;
        self_total_score += self_round_score;

        // Reset round score
        opp_round_score = 0;
        self_round_score = 0;
    }

    println!("Opponent total score: {opp_total_score}");
    println!("Self total score: {self_total_score}");

    // Part 2
    println!("Part 2");

    // Loop through each line of the input
    let mut self_total_score: u32 = 0;
    let file = get_input_file();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        // Convert the line into a vector of chars
        let line = line.expect("Unable to read line");
        let chars: Vec<char> = line.chars().collect();

        self_total_score += *self_strategy_score.get(&chars[2]).expect("HashMap issue with strategy map");
        self_total_score += determine_strategy_shape_score(
            *opp_shape_score.get(&chars[0]).expect("HashMap issue with opp shape map"),
            *self_strategy_score.get(&chars[2]).expect("HashMap issue with self shape map")
        );
    }
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

fn determine_rps_win(a: u32, b: u32) -> u32 {
    let rps_outcome_map = HashMap::from(RPS_OUTCOME_MAP);

    match rps_outcome_map.get(&(a, b)) {
        Some(&score) => score,
        None => panic!("couldn't map the outcome!"),
    }
}

fn determine_strategy_shape_score(opp: u32, mine: u32) -> u32 {
    let rps_outcome_map = HashMap::from(RPS_OUTCOME_MAP);

    let mut result: u32 = 0;
    for (&k, &v) in rps_outcome_map.iter() {
        if k.1 == opp && v == mine {
            result = k.0;
            break;
        }
    };
    result
}
