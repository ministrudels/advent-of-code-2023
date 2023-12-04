use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
struct Solution {
    input: Vec<String>,
}

impl Solution {
    fn new(input: Vec<String>) -> Self {
        Self { input }
    }

    pub fn part_1(&self) -> usize {
        0
    }
    pub fn part_2(&self) -> usize {
        0
    }
}

pub fn main() -> io::Result<()> {
    let path = Path::new("src/day_x/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    // Read each line and convert it to a Vec<char>
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let solution = Solution::new(input);

    println!("Part 1");
    println!("{:?}", solution.part_1());
    println!("Part 2");
    println!("{:?}", solution.part_2());

    Ok(())
}
