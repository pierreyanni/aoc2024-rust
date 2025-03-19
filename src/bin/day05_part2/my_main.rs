use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let lines = read_lines("input/day05.txt")?;

    let (map, list) = clean_data(lines);
    
    println!("{:?}", list);

    let mut wrong_seqs = vec![];
    'outer:for numbers in &list {
        for (number, prev_number) in numbers[1..].iter().zip(numbers[..numbers.len() - 1].iter()) {
            if map.contains_key(number) {
                let values = map.get(number).unwrap();
                if values.contains(prev_number) {
                    wrong_seqs.push(numbers.clone());
                    continue 'outer;
                }
            }
        }
    }
    

    // let mut correct_seqs = vec![];
    let mut total = 0;
    for mut seq in wrong_seqs {
        sort_of_bubble_sort(seq.as_mut_slice(), &map);
        // correct_seqs.push(seq);
        total += seq[(seq.len() - 1) / 2].parse::<i32>().unwrap();
    }

    // println!("{:?}", correct_seqs);
    println!("{}", total);
    Ok(())
}

fn sort_of_bubble_sort(arr: &mut [String], map: &HashMap<String, Vec<String>>) {
    let n = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if map.contains_key(&arr[i]) {
                let values = map.get(&arr[i]).unwrap();
                if values.contains(&arr[i-1]) {
                    arr.swap(i - 1, i);
                swapped = true;
                }
            }
        }
    }
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

