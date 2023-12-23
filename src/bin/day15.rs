use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: Option<usize>,
    box_no: usize,
}

impl FromStr for Lens {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((label, focal_length)) = s.split_once("=") {
            let mut hash = 0;
            label.chars().for_each(|ch| compute_hash(&mut hash, ch));
            Ok(Lens {
                label: label.to_string(),
                focal_length: Some(focal_length.parse().expect("valid number")),
                box_no: hash,
            })
        } else if let Some((label, _)) = s.split_once("-") {
            let mut hash = 0;
            label.chars().for_each(|ch| compute_hash(&mut hash, ch));
            Ok(Lens {
                label: label.to_string(),
                focal_length: None,
                box_no: hash,
            })
        } else {
            Err(anyhow!("invalid string"))
        }
    }
}

fn compute_hash(hash: &mut usize, ch: char) {
    let ascii = (ch as u8) as usize;
    *hash += ascii;
    *hash *= 17;
    *hash = *hash % 256;
}

struct BoxMap {
    boxes: HashMap<usize, Vec<Lens>>,
}

impl BoxMap {
    fn new() -> Self {
        let mut map = HashMap::new();
        // fill the map with default values
        for box_no in 0..256 {
            map.insert(box_no, Vec::new());
        }
        BoxMap { boxes: map }
    }

    fn add_lens(&mut self, lens: Lens) {
        if let Some(idx) = self
            .boxes
            .get(&lens.box_no)
            .unwrap()
            .iter()
            .position(|l| l.label == lens.label)
        {
            self.boxes.get_mut(&lens.box_no).unwrap()[idx] = lens.clone();
        } else {
            self.boxes.get_mut(&lens.box_no).unwrap().push(lens);
        }
    }

    fn remove_lens(&mut self, lens: Lens) {
        if let Some(idx) = self
            .boxes
            .get(&lens.box_no)
            .unwrap()
            .iter()
            .position(|l| l.label == lens.label)
        {
            self.boxes.get_mut(&lens.box_no).unwrap().remove(idx);
        }
    }
}

fn main() -> Result<()> {
    let mut data = include_str!("../../data/day15.input").to_string();
    data.pop();
    let steps = data.split(",");
    let hashes = steps.clone().map(|step| {
        let mut hash = 0;
        step.chars().for_each(|ch| compute_hash(&mut hash, ch));
        hash
    });
    println!("Part 1: {}", hashes.sum::<usize>());

    // part 2
    let steps = steps.filter_map(|s| s.parse::<Lens>().ok());
    let mut map = BoxMap::new();
    for lens in steps {
        match lens.focal_length {
            Some(_) => map.add_lens(lens),
            None => map.remove_lens(lens),
        }
    }
    println!(
        "Part 2: {:?}",
        map.boxes
            .iter()
            .filter(|(_, val)| val.len() != 0)
            .map(|(box_no, lens)| lens
                .iter()
                .enumerate()
                .map(|(idx, l)| (box_no.clone() + 1) * (idx + 1) * l.focal_length.unwrap()))
            .flatten()
            .sum::<usize>()
    );

    Ok(())
}
