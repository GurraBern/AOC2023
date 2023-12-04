use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref COLOR_COUNTS: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("red", 12);
        m.insert("green", 13);
        m.insert("blue", 14);

        return m;
    };
}

fn main() {
    let file = File::open("input.txt").unwrap();

    problem_1(file);
    // problem_2(file, color_counts);
}

fn problem_1(file: File) {
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut game_count = 1;
    for line in reader.lines() {
        let rounds = line.unwrap();
        let is_valid = is_valid_rounds(rounds);

        if is_valid {
            result += game_count;
        }

        game_count += 1;
    }

    println!("result: {}", result);
}

fn is_valid_rounds(line: String) -> bool {
    let mut line = line.to_string();
    let mut is_valid = true;
    if let Some(colon_index) = line.find(':') {
        line.drain(..=colon_index);
    }

    let rounds: Vec<&str> = line.split(';').map(|game| game.trim()).collect();
    for round in rounds {
        is_valid = check_round_validity(round);

        if !is_valid {
            break;
        }
    }

    return is_valid;
}

fn check_round_validity(round: &str) -> bool {
    let pairs: Vec<&str> = round.split(',').map(|pair| pair.trim()).collect();
    for pair in pairs {
        let number_and_color: Vec<&str> = pair.split_whitespace().collect();
        let number = number_and_color[0].parse::<i32>().unwrap();
        let color = number_and_color[1];

        if COLOR_COUNTS[color] < number {
            return false;
        }
    }

    return true;
}
