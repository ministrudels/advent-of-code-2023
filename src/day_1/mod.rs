use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_first_digit(line: &str) -> i32 {
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    0 // Return 0 if no digit is found
}

fn get_last_digit(line: &str) -> i32 {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    0 // Return 0 if no digit is found
}

fn get_first_spelt_digit(line: &str) -> i32 {
    let spelt_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as i32;
        } else {
            for (d, val) in spelt_digits.iter().enumerate() {
                if line[i..].starts_with(val) {
                    return (d + 1) as i32;
                }
            }
        }
    }
    0 // Return 0 if no digit is found
}

fn get_last_spelt_digit(line: &str) -> i32 {
    let spelt_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // https://stackoverflow.com/questions/58926347/how-do-i-get-the-index-from-the-beginning-in-a-reversed-iterator-of-chars-of-a-s
    for (mut i, c) in line.chars().rev().enumerate() {
        i = line.len() - i - 1;
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as i32;
        } else {
            for (d, val) in spelt_digits.iter().enumerate() {
                if line[i..].starts_with(val) {
                    return (d + 1) as i32;
                }
            }
        }
    }
    0 // Return 0 if no digit is found
}

pub fn main() -> io::Result<()> {
    let path = Path::new("src/day_1/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Part 1
    let mut sum = 0;
    for line in &input {
        let first_digit = get_first_digit(line);
        let last_digit = get_last_digit(line);
        sum += first_digit * 10 + last_digit;
    }

    println!("{}", sum);

    // Part 2
    let mut sum = 0;
    for line in &input {
        let first_digit = get_first_spelt_digit(line);
        let last_digit = get_last_spelt_digit(line);
        sum += first_digit * 10 + last_digit;
    }

    println!("{}", sum);

    Ok(())
}
