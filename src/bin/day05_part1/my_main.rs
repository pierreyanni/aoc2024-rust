use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let lines = read_lines("input/day05.txt")?;

    let (map, list) = clean_data(lines);
    
    let mut total = 0;
    'outer:for numbers in &list {
        for (number, prev_number) in numbers[1..].iter().zip(numbers[..numbers.len() - 1].iter()) {
            if map.contains_key(number) {
                let values = map.get(number).unwrap();
                if values.contains(prev_number) {
                    continue 'outer;
                }
            }
        }
        println!("{:?}", &numbers);
        total += numbers[(numbers.len() - 1) / 2].parse::<i32>().unwrap();
    };

    
    println!("{:?}", map);
    println!("{:?}", list);
    println!("{}", total);
    
    Ok(())
}

fn clean_data(lines: Vec<String>) -> (HashMap<String, Vec<String>>, Vec<Vec<String>>) {
    let mut map = HashMap::new();
    let mut list = Vec::new();

    for line in lines {
        if line.contains("|") {
            let parts: Vec<String> = line.split('|').map(String::from).collect();
            let key = &parts[0];
            let value = &parts[1];
            if !map.contains_key(key) {
                map.insert(key.clone(), vec![]);
            }
            map.get_mut(key).unwrap().push(value.clone());
        } else if line.contains(",") {
            let v: Vec<String> = line.split(",").map(String::from).collect();
            list.push(v);
        }
    }

    (map, list)
}

fn read_lines(path_str: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path_str);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Collect non-empty lines into a vector
    let non_empty_lines: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if !line.trim().is_empty() {
                Some(line)
            } else {
                None
            }
        })
        .collect();

    // Print non-empty lines
    Ok(non_empty_lines)
}
