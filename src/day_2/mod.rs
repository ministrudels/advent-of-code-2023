use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_cubes(line: &str, colour: &str) -> Vec<i32> {
    let pattern = Regex::new(&format!(r"(\d+)\s*{}", colour)).unwrap();
    pattern
        .captures_iter(line)
        .filter_map(|cap| {
            cap.get(1)
                .and_then(|match_| match_.as_str().parse::<i32>().ok())
        })
        .collect()
}

fn extract_game_id(line: &str) -> i32 {
    Regex::new(r"Game\s+(\d+)")
        .unwrap()
        .captures(line)
        .and_then(|caps| {
            caps.get(1)
                .and_then(|match_| match_.as_str().parse::<i32>().ok())
        })
        .unwrap()
}

fn part_1(input: &Vec<String>) -> i32 {
    let mut counter = 0;
    for line in input {
        let id = extract_game_id(line);
        let reds = find_cubes(line, "red");
        let greens = find_cubes(line, "green");
        let blues = find_cubes(line, "blue");
        let is_reds_less_than = reds.into_iter().all(|x| x <= 12);
        let is_greens_less_than = greens.into_iter().all(|x| x <= 13);
        let is_blues_less_than = blues.into_iter().all(|x| x <= 14);
        if is_reds_less_than && is_greens_less_than && is_blues_less_than {
            counter += id;
        }
    }
    counter
}

fn part_2(input: &Vec<String>) -> i32 {
    let mut counter = 0;
    for line in input {
        let reds = find_cubes(line, "red");
        let greens = find_cubes(line, "green");
        let blues = find_cubes(line, "blue");
        let max_red = reds.into_iter().max().unwrap();
        let max_green = greens.into_iter().max().unwrap();
        let max_blue = blues.into_iter().max().unwrap();
        let power = max_red * max_green * max_blue;
        counter += power;
    }
    counter
}
pub fn main() -> io::Result<()> {
    let path = Path::new("src/day_2/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
    Ok(())
}
