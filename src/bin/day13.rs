use std::{cmp, usize};

use anyhow::Result;
use aoc::{Grid, Position};
use itertools::Itertools;

fn collect_to_num(grid: &Grid<char>, poses: Vec<Position>) -> usize {
    poses
        .iter()
        .map(|pos| if grid.get(&pos) == &'#' { '1' } else { '2' })
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn collect_row(grid: &Grid<char>, row: usize) -> usize {
    let poses = (0..grid.cols).map(|col| aoc::pos!(row, col)).collect_vec();
    collect_to_num(grid, poses)
}

fn collect_col(grid: &Grid<char>, col: usize) -> usize {
    let poses = (0..grid.rows).map(|row| aoc::pos!(row, col)).collect_vec();
    collect_to_num(grid, poses)
}

#[derive(Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

struct PatternGrid {
    cols: Vec<usize>,
    rows: Vec<usize>,
}

impl PatternGrid {
    fn check_reflection_2(&self, check_row: bool, idxs: (usize, usize)) -> usize {
        let items = if check_row { &self.rows } else { &self.cols };
        let len = items.len();
        let row_top = idxs.0 - 1;
        let row_bottom = idxs.1 - 1;
        let mut idx = 0;
        let target_idx = cmp::min(row_top, len - row_bottom - 1);
        let mut total_differences = 0;
        while idx <= target_idx {
            let r1 = items[row_top - idx];
            let r2 = items[row_bottom + idx];
            // locating the smudge ie the numbers with only 1 char different
            let r1 = r1.to_string().chars().collect_vec();
            let r2 = r2.to_string().chars().collect_vec();
            total_differences += (0..r1.len()).filter(|idx| r1[*idx] != r2[*idx]).count();
            idx += 1
        }
        return total_differences;
    }
    fn check_reflection(&self, check_row: bool, idxs: (usize, usize)) -> Option<usize> {
        let items = if check_row { &self.rows } else { &self.cols };
        let len = items.len();
        let row_top = idxs.0 - 1;
        let row_bottom = idxs.1 - 1;
        let mut idx = 0;
        let target_idx = cmp::min(row_top, len - row_bottom - 1);
        while idx <= target_idx {
            let r1 = items[row_top - idx];
            let r2 = items[row_bottom + idx];
            if r1 == r2 {
                idx += 1
            } else {
                return None;
            }
        }
        Some(row_top + 1)
    }

    fn find_reflection(&self, diff: usize) -> Option<Reflection> {
        // try to find vertical reflections
        let raw = (0..self.cols.len()).collect_vec();
        let idxs = raw.windows(2).map(|pair| (pair[0] + 1, pair[1] + 1));
        let mut matches = idxs
            .map(|pair| (pair, self.check_reflection_2(false, pair)))
            .filter(|pair| pair.1 == diff);
        if let Some(num) = matches.next() {
            return Some(Reflection::Vertical(num.0 .0));
        }

        // try to find horizontal reflections
        let raw = (0..self.rows.len()).collect_vec();
        let idxs = raw.windows(2).map(|pair| (pair[0] + 1, pair[1] + 1));
        let mut matches = idxs
            .map(|pair| (pair, self.check_reflection_2(true, pair)))
            .filter(|pair| pair.1 == diff);
        if let Some(num) = matches.next() {
            return Some(Reflection::Horizontal(num.0 .0));
        }
        None
    }
}

fn main() -> Result<()> {
    let mut data = include_str!("../../data/day13.input").to_string();
    data.pop();
    let data = data.split("\n\n");
    let grids = data.filter_map(|s| {
        let mut s = s.to_string();
        s.push('\n');
        s.parse::<Grid<char>>().ok()
    });

    let grids = grids.map(|g| PatternGrid {
        cols: (0..g.cols).map(|col| collect_col(&g, col)).collect(),
        rows: (0..g.rows).map(|row| collect_row(&g, row)).collect(),
    });

    let score: usize = grids
        .clone()
        .filter_map(|g| g.find_reflection(0))
        .map(|reflection| match reflection {
            Reflection::Vertical(num) => num,
            Reflection::Horizontal(num) => num * 100,
        })
        .sum();
    println!("Part 1: {score}");

    let score: usize = grids
        .filter_map(|g| g.find_reflection(1))
        .map(|reflection| match reflection {
            Reflection::Vertical(num) => num,
            Reflection::Horizontal(num) => num * 100,
        })
        .sum();
    println!("Part 2: {score}");

    Ok(())
}
