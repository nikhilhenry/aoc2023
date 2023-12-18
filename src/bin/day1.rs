use anyhow::Result;

fn main() -> Result<()> {
    let data = aoc::read_one_per_line::<String>("./data/day1.input")?;
    println!("{:?}", data);
    Ok(())
}
