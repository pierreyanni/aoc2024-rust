use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input/day01.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        if let (Some(a), Some(b)) = (numbers.next(), numbers.next()) {
            column1.push(a);
            column2.push(b);
        }
    }

    column1.sort_unstable();
    column2.sort_unstable();

    let total_difference: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total difference: {}", total_difference);

    Ok(())
}
