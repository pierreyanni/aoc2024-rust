use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use std::collections::HashMap;
use itertools::Itertools;
use std::time::Instant;

fn read_file_to_string(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_disk_map(contents: String) -> Vec<Option<u32>> {
    contents.trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let count = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                vec![Some(i as u32 / 2); count]
            } else {
                vec![None; count]
            }
        })
        .collect()
}

fn find_first_consecutive_none(disk_map: &[Option<u32>], count: usize, pos: usize) -> Option<usize> {
    disk_map.windows(count)
        .enumerate()
        .take_while(|(i, _)| i < &pos)
        .find(|(_, window)| window.iter().all(|x| x.is_none()))
        .map(|(pos, _)| pos)
}

fn find_first_positions(disk_map: &[Option<u32>], unique_vals: &[u32]) -> HashMap<u32, usize> {
    let mut positions = HashMap::new();
    
    for &val in unique_vals.iter() {
        positions.insert(val, disk_map.iter()
            .enumerate()    
            .filter(|(_, x)| **x == Some(val))
            .map(|(pos, _)| pos)
            .min()
            .unwrap_or(0)
        );
    }
    
    positions
}

fn process_disk_map(disk_map: &mut Vec<Option<u32>>, unique_vals: &[u32], counts: &HashMap<u32, usize>, positions: &HashMap<u32, usize>) {
    for val in unique_vals {
        let count = counts[val];
        let pos = positions[val];
        let first_none = find_first_consecutive_none(disk_map, count, pos);
        if let Some(none_pos) = first_none {
            for i in pos..pos+count {
                disk_map[i] = None;
            }
            for j in none_pos..none_pos+count {
                disk_map[j] = Some(*val);
            }
        }
    }
}

fn calculate_total(disk_map: &[Option<u32>]) -> u64 {
    disk_map.iter()
        .enumerate()
        .map(|(i, &val)| i as u64 * val.unwrap_or_default() as u64)
        .sum()
}

fn get_unique_values_and_counts(disk_map: &[Option<u32>]) -> (Vec<u32>, HashMap<u32, usize>) {
    let mut counts: HashMap<u32, usize> = HashMap::new();
    let mut unique_vals = Vec::new();

    for val in disk_map.iter().filter_map(|&x| x) {
        if !counts.contains_key(&val) {
            unique_vals.push(val);
        }
        *counts.entry(val).or_insert(0) += 1;
    }

    unique_vals.sort_by(|a, b| b.cmp(a)); // Sort in reverse order
    (unique_vals, counts)
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    
    let path = Path::new("input/day09.txt");
    let contents = read_file_to_string(path)?;

    let mut disk_map = create_disk_map(contents);

    let (unique_vals, counts) = get_unique_values_and_counts(&disk_map);
    let positions = find_first_positions(&disk_map, &unique_vals);
    process_disk_map(&mut disk_map, &unique_vals, &counts, &positions);

    let total = calculate_total(&disk_map);
    println!("total: {:?}", total);
    println!("time: {:?}", start.elapsed());

    Ok(())
}
