use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let lines = read_lines("input/day05.txt")?;


    let (dependency_map, sequences) = parse_data(lines);

    let wrong_sequences = find_wrong_sequences(&dependency_map, &sequences);
    let total = calculate_corrections(wrong_sequences, &dependency_map);

    println!("{}", total);
    Ok(())
}

fn find_wrong_sequences(map: &HashMap<String, Vec<String>>, sequences: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut wrong_seqs = vec![];

    'outer: for numbers in sequences {
        for (current, previous) in numbers[1..].iter().zip(&numbers[..numbers.len()-1]) {
            if let Some(values) = map.get(current) {
                if values.contains(previous) {
                    wrong_seqs.push(numbers.clone());
                    continue 'outer;
                }
            }
        }
    }
    wrong_seqs
}

fn calculate_corrections(mut wrong_sequences: Vec<Vec<String>>, map: &HashMap<String, Vec<String>>) -> i32 {
    let mut total = 0;

    for seq in &mut wrong_sequences {
        sort_of_bubble_sort(seq, map);
        if let Ok(value) = seq[seq.len() / 2].parse::<i32>() {
            total += value;
        }
    }
    total
}

fn sort_of_bubble_sort(arr: &mut [String], map: &HashMap<String, Vec<String>>) {
    let n = arr.len();
    
    for _ in 0..n {
        for i in 1..n {
            if let Some(values) = map.get(&arr[i]) {
                if values.contains(&arr[i - 1]) {
                    arr.swap(i - 1, i);
                }
            }
        }
    }
}

fn parse_data(lines: Vec<String>) -> (HashMap<String, Vec<String>>, Vec<Vec<String>>) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut sequences = Vec::new();

    for line in lines {
        if line.contains("|") {
            let parts: Vec<String> = line.split('|').map(String::from).collect();
            map.entry(parts[0].clone()).or_default().push(parts[1].clone());
        } else if line.contains(",") {
            let sequence: Vec<String> = line.split(",").map(String::from).collect();
            sequences.push(sequence);
        }
    }

    (map, sequences)
}

fn read_lines(path_str: &str) -> io::Result<Vec<String>> {
    let file = File::open(Path::new(path_str))?;
    let reader = io::BufReader::new(file);

    // Collect non-empty lines into a vector
    let non_empty_lines = reader.lines()
        .filter_map(|line| line.ok().filter(|l| !l.trim().is_empty()))
        .collect::<Vec<String>>();

    Ok(non_empty_lines)
}