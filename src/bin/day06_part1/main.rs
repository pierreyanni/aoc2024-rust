use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use ndarray::Array2;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Status {
    Moving,
    Stopped,
}

#[derive(Debug)]
struct Position {
    current: (usize, usize),
    direction: Direction,
    visited: Vec<(usize, usize)>,
    map: Array2<char>,
    status: Status,
}

impl Position {
    fn update_position(&mut self) {
        let next = self.get_next_position();

        if next.0 >= self.map.nrows() || next.1 >= self.map.ncols() {
            self.status = Status::Stopped;
        } else if self.map[[next.0, next.1]] == '#' {
            self.change_direction();
            self.update_position();
        } else {
            self.current = next;
            self.visited.push(next);
        }
    }

    fn get_next_position(&self) -> (usize, usize) {
        match self.direction {
            Direction::Up => (self.current.0 - 1, self.current.1),
            Direction::Down => (self.current.0 + 1, self.current.1),
            Direction::Left => (self.current.0, self.current.1 - 1),
            Direction::Right => (self.current.0, self.current.1 + 1),
        }
    }

    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

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

fn find_initial_position(map: &Array2<char>) -> Position {
    let mut start = (0, 0);
    for i in 0..map.nrows() {
        for j in 0..map.ncols() {
            if map[[i, j]] == '^' {
                start = (i, j);
                break;
            }
        }
    }
    Position {
        current: start,
        direction: Direction::Up,
        visited: vec![start],
        map: map.clone(),
        status: Status::Moving,
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("input/day06.txt");
    let file = File::open(path)?;
    let map = create_matrix(file);

    let mut position = find_initial_position(&map);

    while position.status == Status::Moving {
        position.update_position();
    }

    let unique_visited: HashSet<_> = position.visited.into_iter().collect();
    println!("Unique positions visited: {}", unique_visited.len());
    
    Ok(())
}
