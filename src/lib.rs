use anyhow::Result;
use core::str::FromStr;
use std::{collections::HashMap, fmt::Display, fs::read_to_string, ops::Add};

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(read_to_string(path)?
        .split("\n")
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

#[derive(Hash, Eq, PartialEq)]
pub struct Position {
    row: i32,
    col: i32,
}

#[macro_export]
macro_rules! pos {
    ($row:expr, $col:expr) => {
        Position {
            row: $row as i32,
            col: $col as i32,
        }
    };
}

pub struct Grid<T> {
    rows: usize,
    cols: usize,
    nodes: HashMap<Position, T>,
}

impl<T: std::fmt::Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f).unwrap();
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:?} ", self.nodes.get(&pos!(row, col)).unwrap()).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl<T: Default> Grid<T> {
    pub fn new(rows: usize, cols: usize, prefill: bool) -> Self {
        let mut nodes = HashMap::new();
        if prefill {
            // prefill the HashMap
            (0..rows)
                .map(|row| (0..cols).map(move |col| pos!(row as i32, col as i32)))
                .flatten()
                .for_each(|pos| {
                    nodes.insert(pos, T::default());
                });
        }

        Grid { rows, cols, nodes }
    }

    pub fn get_mut(&mut self, pos: &Position) -> &mut T {
        self.nodes.get_mut(pos).expect("invalid position")
    }

    pub fn get_neighbours(&self, me: &Position, diag: bool) -> Vec<Position> {
        if diag {
            DIAG_OFFSETS
                .iter()
                .map(|offset| me + offset)
                .filter(|pos| self.is_valid_pos(pos))
                .collect()
        } else {
            OFFSETS
                .iter()
                .map(|offset| me + offset)
                .filter(|pos| self.is_valid_pos(pos))
                .collect()
        }
    }

    fn is_valid_pos(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < self.rows as i32 && pos.col >= 0 && pos.col < self.cols as i32
    }
}
const OFFSETS: [Position; 4] = [
    pos!(1, 0),  // right
    pos!(-1, 0), // left
    pos!(0, -1), // top
    pos!(0, 1),  // bottom
];

const DIAG_OFFSETS: [Position; 4] = [
    pos!(1, -1),  // top right
    pos!(-1, -1), // top left
    pos!(1, 1),   // bottom right
    pos!(-1, 1),  // bottom left
];

impl Add<&Position> for &Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<&Position> for Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
