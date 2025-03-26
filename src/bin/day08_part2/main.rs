use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use ndarray::Array2;
use itertools::Itertools;

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

fn extract_nodes(map: Array2<char>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..map.nrows() {
        for j in 0..map.ncols() {
            if map[[i, j]] != '.' {
                nodes.entry(map[[i, j]]).or_insert(vec![]).push((i as i32, j as i32));
            }
        }
    }
    nodes
}

fn get_unique_pairs(points: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
    points.iter()
        .combinations(2)
        .map(|v| (*v[0], *v[1]))
        .collect()
}

fn compute_antinodes(pair: ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let mut antinodes = vec![];
    let (p1, p2) = pair;
    antinodes.push((2*p2.0 - p1.0, 2*p2.1 - p1.1)); // d = y - x; y + d = 2y - x
    antinodes.push((2*p1.0 - p2.0, 2*p1.1 - p2.1)); // d = y - x; x - d = 2x - y
    antinodes
}

fn extract_antinodes(nodes: HashMap<char, Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut all_antinodes = vec![];
    for (_, points) in nodes {
        let pairs = get_unique_pairs(&points);
        for pair in pairs {
            let antinodes = compute_antinodes(pair);
            all_antinodes.extend(antinodes);
        }
    }
    all_antinodes
}

fn filter_unique_antinodes(antinodes: Vec<(i32, i32)>, map: Array2<char>) -> Vec<(i32, i32)> {
    antinodes
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < map.nrows() as i32 && *y < map.ncols() as i32)
        .unique()
        .copied()
        .collect()
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let path = Path::new("input/day08.txt");
    let file = File::open(path)?;
    let map = create_matrix(file);

    // println!("{:?}", map);

    let nodes = extract_nodes(map.clone());
    let antinodes = extract_antinodes(nodes);
    let unique_antinodes = filter_unique_antinodes(antinodes, map);

    println!("Number of unique antinodes: {:?}", unique_antinodes.len());

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}

