use anyhow::Result;
use aoc::{Grid, Position};
use itertools::Itertools;

#[derive(Default, Clone)]
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
    fn promote(&mut self, current: &Position, direction: &Position) -> Option<Position>;
    fn tilt(&mut self, direction: &Position);
    fn get_total_load(&self) -> usize;
}

impl RockGrid for Grid<RockType> {
    fn promote(&mut self, current: &Position, direction: &Position) -> Option<Position> {
        let upper = direction + current;
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

    fn tilt(&mut self, direction: &Position) {
        let rows = if direction.row <= 0 {
            (0..self.rows).collect_vec()
        } else {
            (0..self.rows).rev().collect_vec()
        };
        let cols = if direction.col <= 0 {
            (0..self.cols).collect_vec()
        } else {
            (0..self.cols).rev().collect_vec()
        };

        for row in rows {
            for col in cols.clone() {
                let mut current = aoc::pos!(row, col);
                match self.get(&current) {
                    RockType::Rounded => {
                        while let Some(new_pos) = self.promote(&current, direction) {
                            current = new_pos;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn get_total_load(&self) -> usize {
        let mut total_load = 0;

        for (pos, rock) in self.nodes.iter() {
            match rock {
                RockType::Rounded => total_load += self.rows - pos.row as usize,
                _ => (),
            }
        }
        total_load
    }
}

fn main() -> Result<()> {
    let mut grid: Grid<RockType> = include_str!("../../data/day14.example").parse()?;
    let mut grid_2: Grid<RockType> = include_str!("../../data/day14.example").parse()?;

    println!("Intial Grid");
    println!("{grid}");

    grid.tilt(&aoc::pos!(-1, 0));

    //println!("Titled Grid");
    //println!("{grid}");

    //println!("Part 1: {}", grid.get_total_load());

    // part 2
    for _ in 0..3 {
        grid_2.tilt(&aoc::pos!(-1, 0)); // tilt north
        grid_2.tilt(&aoc::pos!(0, -1)); // tilt west
        grid_2.tilt(&aoc::pos!(1, 0)); // tilt south
        grid_2.tilt(&aoc::pos!(0, 1)); // tilt east
    }

    println!("{grid_2}");

    println!("Part 2: {}", grid_2.get_total_load());

    Ok(())
}
