use std::fs::File;
use std::io::{self, BufRead};
use ndarray::Array2;

pub fn create_matrix(file: File) -> Array2<char> {
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
