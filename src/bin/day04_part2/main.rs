use ndarray::Array2;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let matrix = create_matrix("input/day04.txt")?;
    let count = count_patterns(&matrix, 'A', &["MAS", "SAM"]);
    println!("{}", count);
    Ok(())
}

// Reads file and creates a character matrix
fn create_matrix(filename: &str) -> io::Result<Array2<char>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok().map(|line| line.chars().collect()))
        .collect();

    let nrows = rows.len();
    let ncols = rows.first().map_or(0, |row| row.len());
    let data = rows.into_iter().flatten().collect();

    Ok(Array2::from_shape_vec((nrows, ncols), data).expect("Error creating matrix"))
}

// Counts patterns around a target character
fn count_patterns(matrix: &Array2<char>, target: char, patterns: &[&str]) -> usize {
    let mut count = 0;
    let nrows = matrix.nrows();
    let ncols = matrix.ncols();

    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            if matrix[(i, j)] == target {
                let diags = extract_diagonals(matrix, i, j);
                if patterns.contains(&diags[0].as_str()) && patterns.contains(&diags[1].as_str()) {
                    count += 1;
                }
            }
        }
    }
    count
}

// Extracts both diagonals from a given position
fn extract_diagonals(matrix: &Array2<char>, i: usize, j: usize) -> [String; 2] {
    [
        extract_diagonal(matrix, i, j, true),
        extract_diagonal(matrix, i, j, false),
    ]
}

// Extracts a 3-character diagonal from a position
fn extract_diagonal(matrix: &Array2<char>, i: usize, j: usize, main: bool) -> String {
    let offsets = [-1, 0, 1];
    offsets
        .iter()
        .map(|&d| {
            let (r, c) = if main {
                (i.wrapping_add(d as usize), j.wrapping_add(d as usize))
            } else {
                (i.wrapping_add(d as usize), j.wrapping_sub(d as usize))
            };
            matrix[(r, c)]
        })
        .collect()
}
