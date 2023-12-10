use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    problem_1();
    problem_2();
}

fn problem_1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let mut line = line.unwrap().to_string();
        if let Some(colon_index) = line.find(':') {
            line.drain(..=colon_index);
        }

        let (winning, card) = line.split_once(" | ").unwrap();
        let prize = calculate_winnings(winning.trim().to_string(), card.trim().to_string());
        result += prize;
    }

    println!("Problem 1 answer: {}", result);
}

fn problem_2() {



    let mut result = 0;

    println!("Problem 2 answer: {}", result);
}

fn calculate_winnings(winning_numbers: String, card_numbers: String) -> i32 {
    let win_numbers: Vec<&str> = winning_numbers.split(" ").collect();
    let numbers: Vec<&str> = card_numbers.split(" ").collect();

    let win_numbers: Vec<i32> = win_numbers
        .iter()
        .map(|&s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    let numbers: Vec<i32> = numbers
        .iter()
        .map(|&s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    let mut i: i32 = 0;
    let mut score: i32 = 0;
    for &number in &numbers {
        if win_numbers.iter().any(|&x| x == number) {
            score = 2_i32.pow(i as u32);
            i += 1;
        }
    }

    return score;
}