use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input/day01.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        if let (Some(a), Some(b)) = (numbers.next(), numbers.next()) {
            left_list.push(a);
            right_list.push(b);
        }
    }

    // Count occurrences of each number in the right list
    let mut right_count = HashMap::new();
    for &num in &right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let mut similarity_score = 0;
    for &num in &left_list {
        if let Some(&count) = right_count.get(&num) {
            similarity_score += num * count;
        }
    }

    println!("Similarity score: {}", similarity_score);

    Ok(())
}
