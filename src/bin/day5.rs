use anyhow::{anyhow, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, usize};

struct Range {
    source_start: usize,
    source_end: usize,
    dest: usize,
}

impl Range {
    fn from_vec(data: Vec<usize>) -> Result<Self> {
        if data.len() == 0 {
            return Err(anyhow!("empty list"));
        }
        Ok(Self::new(data[0], data[1], data[2]))
    }
    fn new(dest: usize, source: usize, length: usize) -> Self {
        Range {
            source_start: source,
            source_end: source + length - 1,
            dest,
        }
    }
    fn contains(&self, val: usize) -> bool {
        self.source_start <= val && val <= self.source_end
    }
    fn get_dest(&self, val: usize) -> usize {
        let diff = val - self.source_start;
        self.dest + diff
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn get_dest(&self, source: usize) -> usize {
        if let Some(range) = self
            .ranges
            .iter()
            .filter(|range| range.contains(source))
            .next()
        {
            return range.get_dest(source);
        } else {
            return source;
        }
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let ranges = s
            .split("\n")
            .skip(1)
            .filter_map(|line| {
                Range::from_vec(
                    line.split(" ")
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect(),
                )
                .ok()
            })
            .collect();

        Ok(Map { ranges })
    }
}

fn main() -> Result<()> {
    let mut data = include_str!("../../data/day5.input").split("\n\n");

    let seeds_data = data.next().unwrap();
    let seeds_1 = seeds_data
        .split(" ")
        .filter_map(|num| num.parse::<usize>().ok())
        .collect_vec();

    let seed_to_soil = data.next().unwrap().parse::<Map>().unwrap();
    let soil_to_fertilizer = data.next().unwrap().parse::<Map>().unwrap();
    let fertilizer_to_water = data.next().unwrap().parse::<Map>().unwrap();
    let water_to_light = data.next().unwrap().parse::<Map>().unwrap();
    let light_to_temp = data.next().unwrap().parse::<Map>().unwrap();
    let temp_to_humid = data.next().unwrap().parse::<Map>().unwrap();
    let humid_to_location = data.next().unwrap().parse::<Map>().unwrap();

    let seeds = seeds_1
        .iter()
        .map(|seed| {
            let id = *seed;
            let soil = seed_to_soil.get_dest(id);
            let fertilizer = soil_to_fertilizer.get_dest(soil);
            let water = fertilizer_to_water.get_dest(fertilizer);
            let light = water_to_light.get_dest(water);
            let temp = light_to_temp.get_dest(light);
            let humidity = temp_to_humid.get_dest(temp);
            let location = humid_to_location.get_dest(humidity);

            location
        })
        .collect_vec();

    println!("Part 1: {:?}", seeds.iter().sorted().next().unwrap());

    let new_seeds = seeds_1
        .chunks(2)
        .map(|pair| (pair[0]..pair[0] + pair[1]).collect_vec())
        .flatten()
        .collect_vec();

    println!("Number of seeds: {:?}", new_seeds.len());

    let seeds: Vec<usize> = new_seeds
        .par_iter()
        .map(|seed| {
            let id = *seed;
            let soil = seed_to_soil.get_dest(id);
            let fertilizer = soil_to_fertilizer.get_dest(soil);
            let water = fertilizer_to_water.get_dest(fertilizer);
            let light = water_to_light.get_dest(water);
            let temp = light_to_temp.get_dest(light);
            let humidity = temp_to_humid.get_dest(temp);
            let location = humid_to_location.get_dest(humidity);

            location
        })
        .collect();

    println!("Part 2: {:?}", seeds.iter().sorted().next().unwrap());

    Ok(())
}
