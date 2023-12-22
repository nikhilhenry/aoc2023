use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Seed {
    id: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temp: usize,
    humidity: usize,
    location: usize,
}

fn process_range(dest_start: usize, source_start: usize, length: usize) -> Vec<(usize, usize)> {
    let mut range_map = Vec::new();

    for idx in 0..length {
        range_map.push((source_start + idx, dest_start + idx))
    }

    range_map
}
fn main() -> Result<()> {
    let mut data = include_str!("../../data/day5.example").split("\n\n");

    let seeds = data
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|num| num.parse::<usize>().ok())
        .collect_vec();

    println!("{:?}", seeds);

    let mut seed_to_soil: HashMap<usize, usize> = HashMap::new();
    create_map(&mut data, &mut seed_to_soil);

    let mut soil_to_fertilizer: HashMap<usize, usize> = HashMap::new();
    create_map(&mut data, &mut soil_to_fertilizer);

    let mut fertilizer_to_water: HashMap<usize, usize> = HashMap::new();
    create_map(&mut data, &mut fertilizer_to_water);

    let mut water_to_light: HashMap<usize, usize> = HashMap::new();
    create_map(&mut data, &mut water_to_light);

    let mut light_to_temp: HashMap<usize, usize> = HashMap::new();
    create_map(&mut data, &mut light_to_temp);

    let mut temp_to_humidity: HashMap<usize, usize> = HashMap::new();
    create_map(&mut data, &mut temp_to_humidity);

    let mut humidity_to_location: HashMap<usize, usize> = HashMap::new();
    data.next()
        .unwrap()
        .split("\n")
        .skip(1)
        .collect_vec()
        .iter()
        .rev()
        .skip(1)
        .for_each(|s| {
            let mut vals = s.split(" ").filter_map(|num| num.parse::<usize>().ok());
            process_range(
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            )
            .iter()
            .for_each(|(source, dest)| {
                humidity_to_location.insert(*source, *dest);
            })
        });

    // please extract the above to a method
    let seeds = seeds
        .iter()
        .map(|id| {
            let soil = *seed_to_soil.get(id).unwrap_or(id);
            let fertilizer = *soil_to_fertilizer.get(&soil).unwrap_or(&soil);
            let water = *fertilizer_to_water.get(&fertilizer).unwrap_or(&fertilizer);
            let light = *water_to_light.get(&water).unwrap_or(&water);
            let temp = *light_to_temp.get(&light).unwrap_or(&light);
            let humidity = *temp_to_humidity.get(&temp).unwrap_or(&temp);
            let location = *humidity_to_location.get(&humidity).unwrap_or(&humidity);

            Seed {
                id: *id,
                soil,
                fertilizer,
                water,
                light,
                temp,
                humidity,
                location,
            }
        })
        .collect_vec();

    //println!("{:?}", seeds);

    println!(
        "Part 1: {:?}",
        seeds
            .iter()
            .sorted_by_key(|seed| seed.location)
            .next()
            .unwrap()
            .location
    );

    Ok(())
}

fn create_map(data: &mut std::str::Split<'_, &str>, map: &mut HashMap<usize, usize>) {
    data.next().unwrap().split("\n").skip(1).for_each(|s| {
        let mut vals = s.split(" ").filter_map(|num| num.parse::<usize>().ok());
        process_range(
            vals.next().unwrap(),
            vals.next().unwrap(),
            vals.next().unwrap(),
        )
        .iter()
        .for_each(|(source, dest)| {
            map.insert(*source, *dest);
        })
    });
}
