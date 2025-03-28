use std::path;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::time::Instant;
use itertools::Itertools;

const MULTIPLIER: i64 = 2024;
const ZERO: &str = "0";
const ONE: &str = "1";

type Element = String;
type Elements = Vec<Element>;

/// Processes a list of elements according to the rules:
/// - "0" becomes "1"
/// - Even-length strings are split in half
/// - Odd-length strings are multiplied by 2024
fn process_elements(elements: &Elements) -> Elements {
    elements.iter().flat_map(|element| {
        match element.as_str() {
            ZERO => vec![ONE.to_string()],
            s if s.len() % 2 == 0 => {
                let (first, second) = s.split_at(s.len() / 2);
                vec![first.to_string(), second.parse::<i64>().unwrap().to_string()]
            }
            s => vec![(s.parse::<i64>().unwrap() * MULTIPLIER).to_string()]
        }
    }).collect()
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    
    // Read input
    let path = path::Path::new("input/day11.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    let mut elements: Elements = line.split_whitespace().map(String::from).collect();

    // Process elements for specified number of iterations
    const ITERATIONS: u32 = 25;
    for _ in 0..ITERATIONS {
        elements = process_elements(&elements);
    }
    
    println!("Result: {}", elements.len());
    println!("Time taken: {:?}", start_time.elapsed());
    
    Ok(())
}
