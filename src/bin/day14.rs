use anyhow::Result;
use aoc::{Grid, Position};
use itertools::Itertools;

#[derive(Default, PartialEq, Clone)]
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
    fn cycle(&mut self);
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

    fn cycle(&mut self) {
        self.tilt(&aoc::pos!(-1, 0)); // tilt north
        self.tilt(&aoc::pos!(0, -1)); // tilt west
        self.tilt(&aoc::pos!(1, 0)); // tilt south
        self.tilt(&aoc::pos!(0, 1)); // tilt east
    }
}

fn main() -> Result<()> {
    let mut grid: Grid<RockType> = include_str!("../../data/day14.input").parse()?;
    let mut grid_2: Grid<RockType> = include_str!("../../data/day14.input").parse()?;

    grid.tilt(&aoc::pos!(-1, 0));

    println!("Part 1: {}", grid.get_total_load());

    // part 2
    let mut cycles_states = Vec::new();
    while !cycles_states.contains(&format!("{grid_2}")) {
        cycles_states.push(format!("{grid_2}"));
        grid_2.cycle();
    }

    let len = cycles_states.len();
    println!("Found cycle in {len} iterations");
    let cycle_start = cycles_states
        .iter()
        .position(|s| s == &format!("{grid_2}"))
        .unwrap();
    println!("Cycle starts at {cycle_start} iteration");
    let cycle_length = len - cycle_start;
    println!("cycle length: {cycle_length}");

    let num_cycles = (1000000000 - len) % cycle_length;

    for _ in 0..num_cycles {
        grid_2.cycle();
    }

    println!("Part 2: {}", grid_2.get_total_load());

    Ok(())
}
