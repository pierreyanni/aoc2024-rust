/*
recursive approach:
* state is position
* get height of current position
* move left, right, up, down but check if possible and height is current + 1
* if height is 9, return final position
*/

use std::fs::File;
use std::io;
use std::path::Path;
use std::time::Instant;
use ndarray::Array2;
use itertools::Itertools;
mod utils;
use utils::create_matrix;

fn is_valid_position(pos: (i32, i32), rows: i32, cols: i32) -> bool {
    pos.0 >= 0 && pos.0 < rows && pos.1 >= 0 && pos.1 < cols
}

fn get_next_positions(current: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (current.0 + 1, current.1),
        (current.0 - 1, current.1),
        (current.0, current.1 + 1),
        (current.0, current.1 - 1),
    ]
}

fn hike(map: &Array2<char>, current_position: (i32, i32), path: &mut Vec<(i32, i32)>, all_paths: &mut Vec<Vec<(i32, i32)>>) {
    path.push(current_position);
    let height = map[(current_position.0 as usize, current_position.1 as usize)].to_digit(10).unwrap() as i32;

    if height == 9 {
        all_paths.push(path.clone());
        path.pop();
        return;
    }

    for next_position in get_next_positions(current_position) {
        if !is_valid_position(next_position, map.nrows() as i32, map.ncols() as i32) {
            continue;
        }
        
        let next_height = map[(next_position.0 as usize, next_position.1 as usize)].to_digit(10).unwrap() as i32;
        if next_height == height + 1 {
            hike(map, next_position, path, all_paths);
        }
    }
    path.pop();
}

fn find_all_paths(map: &Array2<char>) -> Vec<Vec<(i32, i32)>> {
    let mut path = Vec::new();
    let mut all_paths = Vec::new();

    for i in 0..map.nrows() {
        for j in 0..map.ncols() {
            if map[(i, j)] == '0' {
                hike(map, (i as i32, j as i32), &mut path, &mut all_paths);
            }
        }
    }
    all_paths
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let path = Path::new("input/day10.txt");
    let file = File::open(path)?;
    let map = create_matrix(file);

    let all_paths = find_all_paths(&map);
    let unique_start_ends = all_paths.iter()
        .map(|path| [path.first(), path.last()])
        .unique()
        .collect::<Vec<_>>();

    println!("sum of scores trailheads: {:?}", unique_start_ends.len());
    println!("number of distinct trailheads: {:?}", all_paths.len());
    println!("Time taken: {:?}", start_time.elapsed());

    Ok(())
}