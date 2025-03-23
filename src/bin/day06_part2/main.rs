use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use ndarray::Array2;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Loop,
}

#[derive(Debug)]
struct Position {
    current: (i32, i32),
    direction: Direction,
    visited: HashSet<(i32, i32, Direction)>,
    map: Array2<char>,
    status: Status,
}

impl Position {
    fn update_position(&mut self) {
        let next = self.get_next_position();

        if self.is_out_of_bounds(next) {
            self.status = Status::Stopped;
        } else if self.is_obstacle(next) {
            self.direction = self.change_direction();
            self.update_position();
        } else {
            self.move_to_next(next);
        }
    }

    fn get_next_position(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.current.0 - 1, self.current.1),
            Direction::Down => (self.current.0 + 1, self.current.1),
            Direction::Left => (self.current.0, self.current.1 - 1),
            Direction::Right => (self.current.0, self.current.1 + 1),
        }
    }

    fn is_out_of_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= self.map.nrows() as i32 || pos.1 >= self.map.ncols() as i32
    }

    fn is_obstacle(&self, pos: (i32, i32)) -> bool {
        matches!(self.map[[pos.0 as usize, pos.1 as usize]], '#' | 'O')
    }

    fn move_to_next(&mut self, pos: (i32, i32)) {
        self.current = pos;
        if !self.visited.insert((pos.0, pos.1, self.direction.clone())) {
            self.status = Status::Loop;
        }
    }

    fn change_direction(&mut self) -> Direction {
        let directions = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
        let idx = directions.iter().position(|d| *d == self.direction).unwrap();
        directions[(idx + 1) % directions.len()].clone()
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
    let start = map.indexed_iter()
        .find(|(_, c)| **c == '^')
        .map(|((i, j), _)| (i as i32, j as i32))
        .unwrap_or((0, 0));

    Position {
        current: start,
        direction: Direction::Up,
        visited: HashSet::from([(start.0, start.1, Direction::Up)]),
        map: map.clone(),
        status: Status::Moving,
    }
}

fn check_for_loop(new_map: &Array2<char>) -> bool {
    let mut position = find_initial_position(new_map);

    while position.status == Status::Moving {
        position.update_position();
        if position.status == Status::Loop {
            return true;
        }
    }
    false
}

fn process_map(map: &Array2<char>) -> usize {
    let mut position = find_initial_position(map);

    while position.status == Status::Moving {
        position.update_position();
    }

    let potential_obstructions = position.visited
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<_>>();

    let mut loop_count = 0;

    for (i, j) in potential_obstructions {
        if map[[i as usize, j as usize]] == '.' {
            let mut new_map = map.clone();
            new_map[[i as usize, j as usize]] = 'O';

            if check_for_loop(&new_map) {
                loop_count += 1;
            }
        }
    }

    loop_count
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let path = Path::new("input/day06.txt");
    let file = File::open(path)?;
    let map = create_matrix(file);

    let loop_count = process_map(&map);

    println!("Loop count: {}", loop_count);

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}
