use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

/// Type alias for better readability
type Number = i64;

/// Applies the given operator to two numbers
fn apply_operator(a: Number, b: Number, operator: char) -> Number {
    match operator {
        '+' => a + b,
        '*' => a * b,
        '|' => format!("{}{}", a, b).parse().unwrap(),
        _ => panic!("Unsupported operator: {}", operator),
    }
}

/// Generates all possible combinations of operators for a given length
fn generate_combinations(length: usize) -> Vec<String> {
    let mut combinations = Vec::new();
    let operators = ['*', '+', '|'];

    fn generate_recursive(current: &mut String, length: usize, operators: &[char], combinations: &mut Vec<String>) {
        if current.len() == length {
            combinations.push(current.clone());
            return;
        }

        for &op in operators {
            current.push(op);
            generate_recursive(current, length, operators, combinations);
            current.pop();
        }
    }

    generate_recursive(&mut String::new(), length, &operators, &mut combinations);
    combinations
}

/// Parses input file into targets and number lists
fn parse_data(file: File) -> io::Result<(Vec<Number>, Vec<Vec<Number>>)> {
    let mut targets = Vec::new();
    let mut number_lists = Vec::new();

    for line in io::BufReader::new(file).lines().filter_map(Result::ok) {
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(':').collect();
        let target = parts[0].parse::<Number>().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse target: {}", e))
        })?;

        let numbers: Vec<Number> = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<Number>().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse number: {}", e))
            }))
            .collect::<Result<Vec<Number>, io::Error>>()?;

        targets.push(target);
        number_lists.push(numbers);
    }

    Ok((targets, number_lists))
}

/// Computes the sum of all matching targets
fn compute_sum(targets: Vec<Number>, number_lists: Vec<Vec<Number>>) -> Number {
    targets
        .iter()
        .zip(number_lists.iter())
        .filter_map(|(&target, numbers)| {
            generate_combinations(numbers.len() - 1)
                .iter()
                .find(|&combination| {
                    let total = combination
                        .chars()
                        .enumerate()
                        .fold(numbers[0], |acc, (i, op)| {
                            apply_operator(acc, numbers[i + 1], op)
                        });
                    total == target
                })
                .map(|_| target)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let time = Instant::now();

    let path = Path::new("input/day07.txt");
    let file = File::open(path)?;
    let (targets, number_lists) = parse_data(file)?;

    let sum = compute_sum(targets, number_lists);
    println!("Result: {}", sum);
    println!("Time taken: {:?}", time.elapsed());

    Ok(())
}
