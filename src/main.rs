use advent_of_code_2022_04::count_fully_inclusive_pairs;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    score = count_fully_inclusive_pairs(&contents);
    println!("The outcome score is: {score}");
}
