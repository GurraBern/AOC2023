use std::{io::{BufReader, BufRead}, fs::File, io};

const NUMBERS_TEXT_ARR: [(&str, &str); 9] = [("1", "one"), ("2", "two"), ("3", "three"), ("4", "four"), ("5", "five"), ("6", "six"), ("7", "seven"), ("8", "eight"), ("9", "nine")];

fn main() {
    let file = match File::open("../input.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    let file2 = match File::open("../input.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    problem_1(file);
    problem_2(file2);
}

fn problem_1(file: File) {
    let mut result = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let result_number = check_for_numbers(line);
                if let Ok(parsed_number) = result_number.parse::<i32>() {
                    result += parsed_number;
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    println!("Problem 1: {}", result);
}

fn problem_2(file: File) {
    let mut result = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let result_number = check_for_numbers_text(line);
                if let Ok(parsed_number) = result_number.parse::<i32>() {
                    result += parsed_number;
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    println!("Problem 2: {}", result);
}

fn text_to_number(text: &str) -> Option<u32> {
    for (number, number_text) in NUMBERS_TEXT_ARR.iter() {
        if text == *number_text {
            return Some(number.parse().unwrap());
        }
    }
    None
}

fn check_for_numbers(text: String) -> String {
    let mut result = String::new();

    if let Some(c) = text.chars().find(|&c| c.is_numeric()) {
        result.push(c);
    }

    if let Some(c) = text.chars().rev().find(|&c| c.is_numeric()) {
        result.push(c);
    }

    return result;
}

fn check_for_numbers_text(mut line: String) -> String {
    //Replace numeric values with their number in text format
    for number in NUMBERS_TEXT_ARR {
        line = line.replace(number.0, number.1);
    }

    let mut min_index:i32= -1;
    let mut max_index:i32= -1;
    let mut min_number = "";
    let mut max_number = "";

    for number_text in NUMBERS_TEXT_ARR {
        if let Some(index) = line.find(number_text.1) {

            if index as i32 <= min_index || min_index == -1 {
                min_index = index as i32;
                min_number = number_text.1;
            }

            if index as i32 > max_index || max_index == -1 {
                max_index = index as i32;
                max_number = number_text.1;
            }
        }
    }

    for number_text in NUMBERS_TEXT_ARR {
        if let Some(index) = line.rfind(number_text.1) {

            if index as i32 <= min_index || min_index == -1 {
                min_index = index as i32;
                min_number = number_text.1;
            }

            if index as i32 > max_index || max_index == -1 {
                max_index = index as i32;
                max_number = number_text.1;
            }
        }
    }

    let parsed_min_number = text_to_number(min_number).unwrap().to_string();
    let parsed_max_number = text_to_number(max_number).unwrap().to_string();
    let result = check_for_numbers(parsed_min_number.to_owned() + &parsed_max_number);
    return result;
}

fn read_lines(file: &File) -> io::Result<Vec<String>> {
    let reader = BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();
    return lines;
}
