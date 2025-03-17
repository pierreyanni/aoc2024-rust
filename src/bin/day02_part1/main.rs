/*
1) Read the input file
2) get the line into a vector of integers
3) find the differences for the vector
4) check if the differneces are all between 1 and 3 or -1 and -3

*/

fn main() {
    let input = std::fs::read_to_string("input/day02.txt").expect("Failed to read input file");
    let mut sum = 0;

    for line in input.split("\n") {

        let numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if numbers.len() < 2 {
            continue
        }

        let diff =  numbers
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<i32>>();

        if diff
            .iter()
            .all(|&x| (1..=3).contains(&x)) {
            sum += 1;
        } else if diff.clone()
            .iter()
            .all(|&x| (-3..=-1).contains(&x)) {
            sum += 1;
        }

    }
    println!("{}", sum);
}
