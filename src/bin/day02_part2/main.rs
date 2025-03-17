use std::io::{self, BufRead};
use std::fs::File;

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}


fn main() -> io::Result<()> {
    let input = read_lines("input/day02.txt")?;
    
    let mut sum = 0;

    for line in &input {
        let numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if numbers.len() < 2 {
            continue;
        }

        'outer: for index in 0..numbers.len() {
            let short_numbers = numbers
                .iter()
                .enumerate()
                .filter_map(|(i, &num)| if i == index { None } else { Some(num) })
                .collect::<Vec<i32>>();

            let is_increasing = short_numbers
                .windows(2)
                .map(|w| w[1] - w[0])
                .all(|x| (1..=3).contains(&x));

            let is_decreasing = short_numbers
                .windows(2)
                .map(|w| w[1] - w[0])
                .all(|x| (-3..=-1).contains(&x));

            if is_increasing || is_decreasing {
                sum += 1;
                break 'outer;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
