use std::{cmp, usize};

use anyhow::Result;
use aoc::{Grid, Position};
use itertools::Itertools;

fn collect_to_num(grid: &Grid<char>, poses: Vec<Position>) -> usize {
    poses
        .iter()
        .map(|pos| if grid.get(&pos) == &'#' { '1' } else { '0' })
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
    id: usize,
    cols: Vec<usize>,
    rows: Vec<usize>,
}

impl PatternGrid {
    fn check_reflection(&self, check_row: bool, idxs: (usize, usize)) -> Option<usize> {
        let items = if check_row { &self.rows } else { &self.cols };
        let len = items.len();
        let row_top = idxs.0 - 1;
        let row_bottom = idxs.1 - 1;
        let mut idx = 0;
        let target_idx = cmp::min(row_top, len - row_bottom - 1);
        //let target_idx = cmp::max(row_top, len - row_bottom - 1);
        //let target_idx = row_top;
        while idx <= target_idx {
            let r1 = items[row_top - idx];
            let r2 = items[row_bottom + idx];
            if r1 == r2 {
                idx += 1
            } else {
                //println!("t:{row_top} b:{row_bottom} idx: {idx} r1:{r1} r2:{r2}");
                //println!("{:?}", items);
                return None;
            }
        }
        Some(row_top + 1)
    }

    fn find_reflection(&self) -> Option<Reflection> {
        // try to find vertical reflections
        let mut raw = (0..self.cols.len()).collect_vec();
        let idxs = raw.windows(2).map(|pair| (pair[0] + 1, pair[1] + 1));
        let mut matches = idxs.filter_map(|pair| self.check_reflection(false, pair));
        if let Some(num) = matches.next() {
            return Some(Reflection::Vertical(num));
        }

        // try to find horizontal reflections
        let mut raw = (0..self.rows.len()).collect_vec();
        let idxs = raw.windows(2).map(|pair| (pair[0] + 1, pair[1] + 1));
        let mut matches = idxs.filter_map(|pair| self.check_reflection(true, pair));
        if let Some(num) = matches.next() {
            return Some(Reflection::Horizontal(num));
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

    let mut grids = grids.enumerate().map(|(idx, g)| PatternGrid {
        id: idx,
        cols: (0..g.cols).map(|col| collect_col(&g, col)).collect(),
        rows: (0..g.rows).map(|row| collect_row(&g, row)).collect(),
    });

    //println!("{:?}", grids.collect_vec()[0].find_reflection());

    //todo!();

    println!(
        "{:?}",
        grids.clone().map(|g| g.find_reflection()).collect_vec()
    );

    let score: usize = grids
        .filter_map(|g| g.find_reflection())
        .map(|reflection| match reflection {
            Reflection::Vertical(num) => num,
            Reflection::Horizontal(num) => num * 100,
        })
        .sum();
    println!("Part 1: {score}");

    todo!();

    let first_col_ref = grids.next().unwrap().check_reflection(false, (5, 6));
    println!("{:?}", first_col_ref);
    let first_row_ref = grids.next().unwrap().check_reflection(true, (4, 5));
    println!("{:?}", first_row_ref);

    Ok(())
}
