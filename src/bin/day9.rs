use anyhow::Result;
use itertools::Itertools;

fn compute_difference(seq: &Vec<i32>) -> Vec<i32> {
    seq.windows(2).map(|nums| nums[1] - nums[0]).collect_vec()
}

fn process_history(history: Vec<i32>) -> i32 {
    let diff = compute_difference(&history);
    if diff.iter().filter(|el| el != &&0).count() == 0 {
        return history.last().unwrap().clone();
    } else {
        return history.last().unwrap().clone() + process_history(diff);
    }
}

fn main() -> Result<()> {
    let data = include_str!("../../data/day9.input");
    let result: i32 = data
        .split("\n")
        .map(|history| {
            history
                .split(" ")
                .filter_map(|num| num.parse::<i32>().ok())
                .collect_vec()
        })
        .filter(|history| history.len() != 0)
        .map(|history| process_history(history))
        .sum();
    println!("Part 1: {result}");
    Ok(())
}
