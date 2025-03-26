use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

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

fn read_file_to_string(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn check_for_numbers(map: &Vec<Option<u32>>, pos: usize) -> bool {
    for el in map.iter().skip(pos) {
        if el.is_some() {
           return true;
        }
    }
    false
}

fn get_last_number(map: &mut Vec<Option<u32>>) -> u32 {
    if let Some((pos, value)) = map.iter().rev().enumerate()
        .find(|(_, el)| el.is_some())
        .map(|(pos, el)| (map.len() - pos - 1, el.unwrap())) {
        map.remove(pos);
        value
    } else {
        100000
    }
}

fn get_next_number(map: &Vec<Option<u32>>, numbers: &mut Vec<Option<u32>>, pos: usize) -> u32 {
    if map[pos].is_some() {
        map[pos].unwrap()
    } else {
        get_last_number(numbers)
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("input/day09.txt");
    let contents = read_file_to_string(path)?;
    let map = create_disk_map(contents);

    let mut numbers = map.clone();
    let mut revised_map = vec![];
    let mut pos = 0;
    while check_for_numbers(&mut numbers, pos) {
        let number = get_next_number(&map, &mut numbers, pos);
        revised_map.push(number);
        pos += 1;
    }
    let total: u64 = revised_map.iter()
        .enumerate()
        .map(|(i, &val)| i as u64 * val as u64)
        .sum();
    println!("total: {:?}", total);

    Ok(())
}
