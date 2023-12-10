use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    problem_1();
    problem_2();
}

fn problem_1() {
    let grid: Vec<Vec<char>> = create_grid();

    let mut result = 0;
    for (row_index,row) in grid.iter().enumerate() {
        for (column_index,&column) in row.iter().enumerate() {
            if !column.is_numeric() && column != '.' {
                let gear_numbers = include_number_positions(&grid, row_index, column_index);
                result += gear_numbers.iter().sum::<i32>();
            }
        }
    }

    println!("Problem 1 Answer:  {}", result);
}

fn problem_2() {
    let grid: Vec<Vec<char>> = create_grid();

    let mut result = 0;
    for (row_index,row) in grid.iter().enumerate() {
        for (column_index,&column) in row.iter().enumerate() {
            if !column.is_numeric() && column != '.' {
                let gear_numbers = include_number_positions(&grid, row_index, column_index);
                if gear_numbers.len() > 1 {
                    result += gear_numbers.iter().fold(1, |acc, x| acc * x);
                }
            }
        }
    }

    println!("Problem 2 Answer:  {}", result);
}

fn find_position(grid: &Vec<Vec<char>>, row: usize, column: usize, direction: isize) -> usize {
    let mut pos = column as isize;
    while pos >= 0 && pos < grid.get(0).unwrap().len() as isize {
        let char = grid[row][pos as usize];
        if !char.is_numeric() {
            if direction == -1 {
                pos -= direction;
            }
            break;
        }
        pos += direction;
    }

    return pos.max(0) as usize
}

fn include_number_positions(grid: &Vec<Vec<char>>, row: usize, column: usize) -> Vec<i32> {
    let number_span_positions: &mut HashMap<usize, Vec<(usize, usize)>> = &mut HashMap::new();
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (row_offset, col_offset) in &directions {
        let new_row = row as isize + *row_offset;
        let new_col = column as isize + *col_offset;

        if new_row >= 0 && new_row < grid.len() as isize &&
            new_col >= 0 && new_col < grid[0].len() as isize {
            let neighbor_value = grid[new_row as usize][new_col as usize];

            if neighbor_value.is_numeric() {
                include_surrounding_numbers(number_span_positions, grid, new_row as usize, new_col as usize);
            }
        }
    }

    let mut gear_numbers: Vec<i32> = vec![];
    for (row_pos, number_spans) in number_span_positions {
        for number_span in number_spans {
            if let Some(row) = grid.get(*row_pos) {
                let start_column = number_span.0;
                let end_column = number_span.1;

                if let Some(substring) = row.get(start_column..end_column) {
                    let substring_string: String = substring.iter().collect();
                    if let Ok(number) = substring_string.parse::<i32>() {
                        gear_numbers.push(number);
                    }
                }
            }
        }
    }

    return gear_numbers;
}

fn include_surrounding_numbers(number_positions: &mut HashMap<usize, Vec<(usize, usize)>>, grid: &Vec<Vec<char>>, row: usize, column: usize) {
    let number_span = find_number_span(grid, row, column);
    if let Some(current_list) = number_positions.get_mut(&row) {
        if !current_list.contains(&(number_span.0, number_span.1)) {
            current_list.push((number_span.0, number_span.1));
        }
    } else {
        number_positions.insert(row, vec![(number_span.0, number_span.1)]);
    }
}

fn find_number_span(grid: &Vec<Vec<char>>, row: usize, column: usize) -> (usize, usize) {
    let start_pos = find_position(grid, row, column, -1);
    let end_pos = find_position(grid, row, column, 1);

    (start_pos, end_pos)
}

// Helper Methods //
fn create_grid() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        let chars: Vec<char> = row.chars().collect();
        grid.push(chars);
    }

    return grid;
}