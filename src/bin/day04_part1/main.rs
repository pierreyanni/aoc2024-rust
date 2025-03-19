use ndarray::Array2;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input/day04.txt");
    let file = File::open(path)?;
    let matrix = create_matrix(file);

    let strings = extract_all_strings(&matrix);
    let total_count = strings.iter().map(|s| count_xmas(s, "XMAS") + count_xmas(s, "SAMX")).sum::<usize>();

    println!("Total count: {}", total_count);
    Ok(())
}

// Reads file and creates a character matrix
fn create_matrix(file: File) -> Array2<char> {
    let reader = io::BufReader::new(file);
    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok().map(|line| line.chars().collect()))
        .collect();

    let nrows = rows.len();
    let ncols = rows.first().map_or(0, |row| row.len());
    let data = rows.into_iter().flatten().collect();

    Array2::from_shape_vec((nrows, ncols), data).expect("Error creating matrix")
}

// Extracts all rows, columns, and diagonals as strings
fn extract_all_strings(matrix: &Array2<char>) -> Vec<String> {
    let mut strings = Vec::new();

    // Rows and columns
    strings.extend(matrix.rows().into_iter().map(|row| row.iter().collect()));
    strings.extend(matrix.columns().into_iter().map(|col| col.iter().collect()));

    // Diagonals
    strings.extend(extract_diagonals(matrix, true));  // Main diagonals
    strings.extend(extract_diagonals(matrix, false)); // Anti-diagonals

    strings
}

// Extracts diagonals or anti-diagonals based on direction
fn extract_diagonals(matrix: &Array2<char>, main: bool) -> Vec<String> {
    let nrows = matrix.nrows();
    let ncols = matrix.ncols();
    let mut diagonals = Vec::new();

    for i in 0..nrows {
        diagonals.push(extract_diagonal(matrix, i, 0, main));
    }
    for j in 1..ncols {
        diagonals.push(extract_diagonal(matrix, if main { 0 } else { nrows - 1 }, j, main));
    }

    diagonals
}

// Extracts a diagonal or anti-diagonal from a matrix
fn extract_diagonal(matrix: &Array2<char>, start_row: usize, start_col: usize, main: bool) -> String {
    let nrows = matrix.nrows();
    let ncols = matrix.ncols();

    let diagonal: Vec<char> = (0..)
        .map(|i| {
            let (r, c) = if main {
                (start_row + i, start_col + i)
            } else {
                (start_row.checked_sub(i).unwrap_or(nrows), start_col + i)
            };
            (r, c)
        })
        .take_while(|&(r, c)| r < nrows && c < ncols)
        .map(|(r, c)| matrix[(r, c)])
        .collect();

    diagonal.into_iter().collect()
}

// Counts occurrences of a substring using a sliding window
fn count_xmas(text: &str, word: &str) -> usize {
    text.as_bytes()
        .windows(word.len())
        .filter(|&window| window == word.as_bytes())
        .count()
}
