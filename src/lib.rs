use anyhow::Result;
use core::str::FromStr;
use std::fs::read_to_string;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(read_to_string(path)?
        .split("\n")
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
