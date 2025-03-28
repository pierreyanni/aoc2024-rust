use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = read_text("input/day03.txt");

    let text = clean_text(&text);

    let vec_mul = extract_mul(&text);

    let mut sum = 0;
    for mul in vec_mul {
        let pairs = extract_pairs(mul);
        sum += pairs[0] * pairs[1];
    }
    println!("{}", sum);

    Ok(())

}

fn clean_text(text: &str) -> String {
    let mut s = text;
    let mut do_word = true;
    let mut text = String::new();
    while !s.is_empty() {
        if do_word {
            let (first_part, rest) = s.split_once("don't()").unwrap_or((&s, ""));
            text.push_str(first_part);
            do_word = false;
            s = rest;
        } else {
            let (_, rest) = s.split_once("do()").unwrap_or((&s, ""));
            do_word = true;
            s = rest;
        }
    }
    text
}


fn read_text(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            String::new() // Return an empty string on error
        }
    }
}

fn extract_mul(text: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap(); // Match 5-letter words

    let mut vec_mul = Vec::new();
    for mul in re.find_iter(&text) {
        vec_mul.push(mul.as_str());
    }
    vec_mul
}

fn extract_pairs(mul: &str) -> [i32; 2] {
    let re = Regex::new(r"\d+").unwrap();
    let mut vec_pairs = vec![];
    for pair in re.find_iter(mul) {
        vec_pairs.push(pair.as_str().parse::<i32>().unwrap());
    }
    vec_pairs.try_into().unwrap()
}


