use std::char;

use anyhow::Result;
use itertools::Itertools;

const COLS: usize = 140; // number of columns in the given grid

#[derive(Copy, Clone, Debug)]
enum RawStates {
    Digit(char),
    Symbol,
    Blank,
}

#[derive(Copy, Clone, Debug)]
enum GridStates {
    Digit(usize),
    Symbol,
    Blank,
}

fn get_idx(row: usize, col: usize) -> usize {
    row * COLS + col
}

const OFFSETS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, -1),
    (-1, -1),
    (1, 1),
    (-1, 1),
];

fn get_neighbours(row: usize, col: usize) -> Vec<usize> {
    let cols = COLS as i32;
    OFFSETS
        .iter()
        .map(|(of_row, of_col)| (row as i32 + of_row, col as i32 + of_col))
        .filter(|(new_row, new_col)| {
            new_row < &cols && *new_row > -1 && new_col < &cols && *new_col > -1
        })
        .map(|(new_row, new_col)| get_idx(new_row as usize, new_col as usize))
        .collect()
}

macro_rules! print_grid {
    ($raw_grid:expr) => {
        for row in 0..COLS {
            for col in 0..COLS {
                let idx = get_idx(row, col);
                print!("{:?} ", $raw_grid[idx]);
            }
            println!();
        }
    };
}

fn main() -> Result<()> {
    let data = include_str!("../../data/day3.input");
    let data = data.split("\n").collect_vec();

    let mut raw_grid = [RawStates::Blank; COLS.pow(2)];

    // processing the input into the raw grid
    data.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            let idx = get_idx(row, col);
            if char.is_digit(10) {
                raw_grid[idx] = RawStates::Digit(char)
            } else if char == '.' {
                raw_grid[idx] = RawStates::Blank
            } else {
                raw_grid[idx] = RawStates::Symbol
            }
        })
    });

    // print_grid!(raw_grid);

    let mut grid = [GridStates::Blank; COLS.pow(2)];

    raw_grid.iter().enumerate().for_each(|(idx, el)| match el {
        RawStates::Digit(_) => grid[idx] = GridStates::Digit(0),
        RawStates::Symbol => grid[idx] = GridStates::Symbol,
        RawStates::Blank => grid[idx] = GridStates::Blank,
    });

    let mut nums: Vec<u32> = Vec::new();

    for row in 0..COLS {
        let mut num_string = String::new();
        let mut num_idxs: Vec<usize> = Vec::new();
        for col in 0..COLS {
            let idx = get_idx(row, col);
            match raw_grid[idx] {
                RawStates::Digit(ch) => {
                    num_idxs.push(idx);
                    num_string.push(ch)
                }
                _ => {
                    if let Ok(num) = num_string.parse::<u32>() {
                        nums.push(num);
                        num_string = String::default();
                        num_idxs
                            .clone()
                            .into_iter()
                            .for_each(|num_idx| grid[num_idx] = GridStates::Digit(nums.len() - 1));
                        num_idxs = Vec::new();
                    }
                }
            }
        }
        // cleaning up any left overs
        if let Ok(num) = num_string.parse::<u32>() {
            nums.push(num);
            num_idxs
                .clone()
                .into_iter()
                .for_each(|num_idx| grid[num_idx] = GridStates::Digit(nums.len() - 1));
        }
    }

    let mut part_number_idxs: Vec<usize> = Vec::new();

    for row in 0..COLS {
        for col in 0..COLS {
            let idx = get_idx(row, col);
            match grid[idx] {
                GridStates::Digit(id) => {
                    if !part_number_idxs.contains(&id) {
                        let neighbours = get_neighbours(row, col);
                        neighbours
                            .iter()
                            .for_each(|neighbour| match grid[*neighbour] {
                                GridStates::Symbol => part_number_idxs.push(id),
                                _ => (),
                            })
                    }
                }
                _ => (),
            }
        }
    }

    let part_sum = part_number_idxs.iter().map(|idx| nums[*idx]).collect_vec();
    println!("Part 1: {:?}", part_sum.iter().sum::<u32>());

    // part 2:: mania!!
    let mut gear_ratios: Vec<u32> = Vec::new();
    for row in 0..COLS {
        for col in 0..COLS {
            let idx = get_idx(row, col);
            match grid[idx] {
                GridStates::Symbol => {
                    let neighbours = get_neighbours(row, col);
                    let mut gear_idxs: Vec<usize> = Vec::new();
                    neighbours
                        .iter()
                        .for_each(|neighbour| match grid[*neighbour] {
                            GridStates::Digit(id) => {
                                if !gear_idxs.contains(&id) {
                                    gear_idxs.push(id)
                                }
                            }
                            _ => (),
                        });
                    if gear_idxs.len() == 2 {
                        gear_ratios.push(gear_idxs.iter().map(|num_idx| nums[*num_idx]).product());
                    }
                }
                _ => (),
            }
        }
    }
    println!("Part 2: {:?}", gear_ratios.iter().sum::<u32>());

    Ok(())
}
