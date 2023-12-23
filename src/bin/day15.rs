use anyhow::Result;

fn compute_hash(hash: &mut usize, ch: char) {
    if ch == '\n' {
        return ();
    }
    let ascii = (ch as u8) as usize;
    *hash += ascii;
    *hash *= 17;
    *hash = *hash % 256;
}

fn main() -> Result<()> {
    let data = include_str!("../../data/day15.input");
    let steps = data.split(",");
    let hashes = steps.map(|step| {
        let mut hash = 0;
        step.chars().for_each(|ch| compute_hash(&mut hash, ch));
        hash
    });
    println!("Part 1: {}", hashes.sum::<usize>());
    Ok(())
}
