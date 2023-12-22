use anyhow::Result;
use itertools::Itertools;
use std::iter::zip;

fn count_distance(time: usize, dist: usize) -> usize {
    let speeds = (1..time).collect_vec();
    let distances = speeds.iter().map(|speed| (time - speed) * speed);
    distances.filter(|d| *d > dist).count()
}

fn main() -> Result<()> {
    let mut data = include_str!("../../data/day6.input").split("\n");
    let times = data
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok());
    let distances = data
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok());

    let count: usize = zip(times, distances)
        .map(|(time, dist)| count_distance(time, dist))
        .product();
    println!("Part 1: {:?}", count);

    Ok(())
}
