use std::path;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::time::Instant;
use std::collections::HashMap;

// Constants
const MULTIPLIER: i64 = 2024;
const ZERO: &str = "0";
const ONE: &str = "1";
const ITERATIONS: u32 = 75;

// Type aliases for better readability
type Element = String;
type ElementCount = usize;
type ElementMap = HashMap<Element, ElementCount>;

/// Processes a single element according to the rules:
/// - "0" becomes "1"
/// - Even-length strings are split in half
/// - Odd-length strings are multiplied by 2024
fn process_element(element: &str, count: ElementCount) -> Vec<(Element, ElementCount)> {
    match element {
        ZERO => vec![(ONE.to_string(), count)],
        s if s.len() % 2 == 0 => {
            let (first, second) = s.split_at(s.len() / 2);
            vec![
                (first.to_string(), count),
                (second.parse::<i64>().unwrap().to_string(), count)
            ]
        }
        s => vec![((s.parse::<i64>().unwrap() * MULTIPLIER).to_string(), count)]
    }
}

/// Processes a map of elements according to the rules
fn process_elements(elements: ElementMap) -> ElementMap {
    elements.into_iter()
        .flat_map(|(element, count)| process_element(&element, count))
        .fold(ElementMap::new(), |mut acc, (element, count)| {
            *acc.entry(element).or_insert(0) += count;
            acc
        })
}

/// Reads the input file and returns the first line as a vector of strings
fn read_input(path: &str) -> io::Result<Vec<Element>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    Ok(line.split_whitespace().map(String::from).collect())
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    
    // Read and process input
    let input = read_input("input/day11.txt")?;
    let mut elements: ElementMap = input.into_iter()
        .fold(ElementMap::new(), |mut acc, element| {
            *acc.entry(element).or_insert(0) += 1;
            acc
        });

    // Process elements for specified number of iterations
    for _ in 0..ITERATIONS {
        elements = process_elements(elements);
    }
    
    println!("Result: {}", elements.values().sum::<ElementCount>());
    println!("Time taken: {:?}", start_time.elapsed());
    
    Ok(())
}
