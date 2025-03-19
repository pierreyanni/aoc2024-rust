use ndarray::Array2;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("input/day04.txt");
    let file = File::open(path)?;

    let matrix = create_matrix(file);

    let nrows = matrix.nrows();
    let ncols = matrix.ncols();

    let mut strings = Vec::new();

    for row in matrix.rows() {
        let string = row.into_iter().collect::<String>();
        strings.push(string);
    }

    for col in matrix.columns() {
        let string = col.into_iter().collect::<String>();
        strings.push(string);
    }

    for i in 0..nrows {
        let diagonal = extract_diagonal(matrix.clone(), i, 0);
        strings.push(diagonal);
    }
    for i in 1..ncols {
        let diagonal = extract_diagonal(matrix.clone(), 0, i);
        strings.push(diagonal);
    }

    let transposed = matrix.t();

    for i in 0..nrows {
        let diagonal = extract_other_diagonal(transposed.to_owned(), i, 0);
        strings.push(diagonal);
    }

    for i in 1..ncols {
        let diagonal = extract_other_diagonal(transposed.to_owned(), nrows - 1, i);
        strings.push(diagonal);
    }


    let mut count = 0;
    for string in strings {
        count += count_xmas(&string, "XMAS");
        count += count_xmas(&string, "SAMX");
    }

    println!("total count:\n{:?}", count);

    Ok(())
}

fn count_xmas(text: &str, word: &str) -> usize {
    let pattern = format!(r"{}", regex::escape(word));
    let re = Regex::new(&pattern).unwrap();

    let count = re.find_iter(text).count();
    count
}

fn extract_diagonal(matrix: Array2<char>, start_row: usize, start_col: usize) -> String {
    let nrows = matrix.nrows();
    let ncols = matrix.ncols();
    let diagonal: Vec<char> = (0..)
        .map(|i| (start_row + i, start_col + i))
        .take_while(|&(r, c)| r < nrows && c < ncols)
        .map(|(r, c)| matrix[(r, c)])
        .collect();

    diagonal.into_iter().collect()
}

fn extract_other_diagonal(matrix: Array2<char>, start_row: usize, start_col: usize) -> String {
    let ncols = matrix.ncols();
    let diagonal: Vec<char> = (0..)
        .map(|i| (start_row.checked_sub(i), start_col + i))
        .take_while(|&(r, c)| r.is_some() && c < ncols)
        .map(|(r, c)| matrix[(r.unwrap(), c)])
        .collect();

    diagonal.into_iter().collect()
}

fn create_matrix(file: File) -> Array2<char> {
    // Open the file
    
    let reader = io::BufReader::new(file);

    // Read all rows as vectors of characters
    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok().map(|line| line.chars().collect()))
        .collect();

    // Get dimensions
    let nrows = rows.len();
    let ncols = rows.first().map_or(0, |row| row.len());

    // Create a flat vector of all characters
    let data: Vec<char> = rows.into_iter().flatten().collect();

    // Create a 2D array
    let matrix = Array2::from_shape_vec((nrows, ncols), data)
        .expect("Error creating matrix");

    matrix
}