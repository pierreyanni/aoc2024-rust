use ndarray::Array2;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input/day04.txt");
    let file = File::open(path)?;
    let matrix = create_matrix(file);

    let n_cols = matrix.ncols();
    let n_rows = matrix.nrows();    
    
    let mut count = 0;
    for i in 1..n_rows - 1 {
        for j in 1..n_cols - 1 {    
            if matrix[(i, j)] == 'A' {
                let diags = extract_diags(&matrix, i, j);
                if (diags[0] == "MAS" || diags[0] == "SAM") && (diags[1] == "MAS" || diags[1] == "SAM") {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn extract_diags(matrix: &Array2<char>, i: usize, j: usize) -> Vec<String> {
    let mut diags = Vec::new();
    diags.push(extract_diagonal(matrix, i, j, true));
    diags.push(extract_diagonal(matrix, i, j, false));
    diags
}

fn extract_diagonal(matrix: &Array2<char>, i: usize, j: usize, main: bool) -> String {
    let indices = if main {
        [(i-1, j-1), (i, j), (i+1, j+1)]
    } else {
        [(i-1, j+1), (i, j), (i+1, j-1)]
    };
    indices.iter().map(|&(r, c)| matrix[(r, c)]).collect()
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
