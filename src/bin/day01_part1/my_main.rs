use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01.txt").expect("Failed to read the file");

    let mut mat = vec![];
    for line in input.lines() {
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        mat.push(row);
    }
    let v0 = extract_col(&mat, 0);
    let v1 = extract_col(&mat, 1);

    let mut sum = 0;
    for i in 0..v0.len() {
        let distance = (v0[i] - v1[i]).abs();
        sum += distance;
    }
    println!("{}", sum);
    
}

fn extract_col(mat: &Vec<Vec<i32>>, col: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = mat.iter().map(|row| row[col]).collect();
    vec.sort();
    vec
}
