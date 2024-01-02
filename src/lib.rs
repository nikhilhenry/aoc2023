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

#[derive(Hash, Debug, Eq, Copy, Clone, PartialEq)]
pub struct Position {
    pub row: i32,
    pub col: i32,
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

#[derive(Clone, PartialEq)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub nodes: HashMap<Position, T>,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> PartialOrd for Grid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let other_str = format!("{other}");
        let me_str = format!("{self}");
        Some(me_str.cmp(&other_str))
    }
}

impl<T: std::default::Default + std::convert::From<char>> FromStr for Grid<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut s = s.to_string();
        s.pop(); // annoying newline character
        let data = s.split("\n");
        let rows = data.clone().count();
        let cols = data.clone().next().unwrap().chars().count();
        let mut grid: Grid<T> = Grid::new(rows, cols, false);
        data.enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                grid.nodes
                    .insert(pos!(row, col), ch.try_into().expect("unable to parse"));
            })
        });
        Ok(grid)
    }
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
        self.nodes
            .get_mut(pos)
            .expect(&format!("{:?} is invalid", pos))
    }
    pub fn get(&self, pos: &Position) -> &T {
        self.nodes.get(pos).expect(&format!("{:?} is invalid", pos))
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

    pub fn get_neighbours_raw(&self, me: &Position) -> Vec<Position> {
        OFFSETS.iter().map(|offset| me + offset).collect()
    }
    pub fn is_valid_pos(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < self.rows as i32 && pos.col >= 0 && pos.col < self.cols as i32
    }
}
pub fn manhattan_distance(from: &Position, to: &Position) -> usize {
    let x_dist = (to.row - from.row).abs();
    let y_dist = (to.col - from.col).abs();
    (x_dist + y_dist) as usize * 4
}
pub const OFFSETS: [Position; 4] = [
    pos!(0, 1),  // right
    pos!(0, -1), // left
    pos!(-1, 0), // top
    pos!(1, 0),  // bottom
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
