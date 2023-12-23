use anyhow::Result;
use aoc::{Grid, Position};

#[derive(Default)]
enum RockType {
    Rounded,
    Cube,
    #[default]
    Empty,
}

impl std::fmt::Debug for RockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockType::Rounded => write!(f, "{}", 'O'),
            RockType::Cube => write!(f, "{}", '#'),
            RockType::Empty => write!(f, "{}", '.'),
        }
    }
}

impl From<char> for RockType {
    fn from(value: char) -> Self {
        match value {
            'O' => RockType::Rounded,
            '#' => RockType::Cube,
            _ => RockType::Empty,
        }
    }
}

trait RockGrid {
    fn promote(&mut self, current: &Position) -> Option<Position>;
}

impl RockGrid for Grid<RockType> {
    fn promote(&mut self, current: &Position) -> Option<Position> {
        let upper = aoc::pos!(-1, 0) + current;
        if !self.is_valid_pos(&upper) {
            return None;
        }
        match self.get(&upper) {
            RockType::Empty => {
                *self.get_mut(current) = RockType::Empty;
                *self.get_mut(&upper) = RockType::Rounded;
                Some(upper)
            }
            _ => None,
        }
    }
}

fn main() -> Result<()> {
    let mut grid: Grid<RockType> = include_str!("../../data/day14.input").parse()?;

    println!("Intial Grid");
    println!("{grid}");

    for row in 1..grid.rows {
        for col in 0..grid.cols {
            let mut current = aoc::pos!(row, col);
            match grid.get(&current) {
                RockType::Rounded => {
                    while let Some(new_pos) = grid.promote(&current) {
                        current = new_pos;
                    }
                }
                _ => (),
            }
        }
    }

    println!("Titled Grid");
    println!("{grid}");

    let mut total_load = 0;

    for (pos, rock) in grid.nodes.iter() {
        match rock {
            RockType::Rounded => total_load += grid.rows - pos.row as usize,
            _ => (),
        }
    }

    println!("Part 1: {total_load}");

    Ok(())
}
