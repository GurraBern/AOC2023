use std::collections::{HashMap, HashSet};
use std::fs::{File, read};
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    problem_1(file);
}

fn problem_1(file: File) {
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = create_grid(reader);

    let mut number_positions: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for (row_index,row) in grid.iter().enumerate() {
        for (column_index,&column) in row.iter().enumerate() {
            if !column.is_numeric() && column != '.' {
                check_neighbouring_tiles(&mut number_positions, &grid, row_index, column_index);
            }
        }
    }

    let mut result = 0;
    for (row_pos, positions) in &number_positions {
        for pos in positions {
            if let Some(row) = grid.get(*row_pos) {
                let start_column = pos.0;
                let end_column = pos.1;

                if let Some(substring) = row.get(start_column..end_column) {
                    let substring_string: String = substring.iter().collect();
                    if let Ok(number) = substring_string.parse::<i32>() {
                        result += number;
                    }
                }
            }
        }
    }

    println!("done {}", result);
}

//TODO Hashmap with row and then tuple with start and end position
fn check_neighbouring_tiles(number_positions: &mut HashMap<usize, Vec<(usize, usize)>>, grid: &Vec<Vec<char>>, row: usize, column: usize) {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for (row_offset, col_offset) in &directions {
        let new_row = row as isize + *row_offset;
        let new_col = column as isize + *col_offset;

        if new_row >= 0 && new_row < grid.len() as isize &&
            new_col >= 0 && new_col < grid[0].len() as isize {
            let neighbor_value = grid[new_row as usize][new_col as usize];

            if neighbor_value.is_numeric() {
                include_directions(number_positions, grid, new_row as usize, new_col as usize);
            }
        }
    }
}

fn include_directions(number_positions: &mut HashMap<usize, Vec<(usize, usize)>>, grid: &Vec<Vec<char>>, row: usize, column: usize) {
    let start_pos = find_position(grid, row, column, -1);
    let end_pos = find_position(grid, row, column, 1);

    if let Some(current_list) = number_positions.get_mut(&row) {
        if !current_list.contains(&(start_pos, end_pos)) {
            current_list.push((start_pos, end_pos));
        }
    } else {
        number_positions.insert(row, vec![(start_pos, end_pos)]);
    }
}

fn find_position(grid: &Vec<Vec<char>>, row: usize, column: usize, direction: isize) -> usize {
    let mut pos = column as isize;
    while pos >= 0 {
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

fn create_grid(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        let chars: Vec<char> = row.chars().collect();
        grid.push(chars);
    }

    grid
}