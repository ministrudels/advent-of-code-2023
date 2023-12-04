use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Solution {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}
impl Solution {
    fn new(mut input: Vec<Vec<char>>) -> Self {
        // Pad top and bottom
        input.insert(0, vec!['.'; input[0].len()]);
        input.push(vec!['.'; input[0].len()]);

        // Pad left and right
        for row in input.iter_mut() {
            row.insert(0, '.');
            row.push('.');
        }

        let rows = input.len();
        let cols = if rows > 0 { input[0].len() } else { 0 };
        Self {
            grid: input,
            rows,
            cols,
        }
    }

    /// For a digit, get the whole digit, and the starting position of the digit
    fn get_number_at(&self, r: usize, c: usize) -> Option<(usize, (usize, usize))> {
        if r >= self.rows || c >= self.cols {
            return None;
        }

        let mut p = c;
        let mut is_start_of_number = false;
        // println!("{}", self.input[r][p]);
        while !is_start_of_number {
            if p == 0 || !self.grid[r][p - 1].is_digit(10) {
                is_start_of_number = true;
            } else {
                p -= 1;
            }
        }
        // p += 1;
        let starting_position = (r, p);
        let mut result = String::new();
        while self.grid[r][p].is_digit(10) {
            result.push(self.grid[r][p]);
            p += 1;
        }

        result
            .parse::<usize>()
            .ok()
            .map(|num| (num, starting_position))
    }

    fn is_symbol(&self, r: usize, c: usize) -> bool {
        !self.grid[r][c].is_digit(10) && self.grid[r][c] != '.'
    }

    fn is_number_next_to_symbol(&self, r: usize, c: usize) -> bool {
        let start = c;
        let mut end = c;
        while end < self.cols {
            if end == self.cols - 1 {
                break;
            }
            if !self.grid[r][end + 1].is_digit(10) {
                break;
            }
            end += 1;
        }

        // loop from start to end
        for i in start..=end {
            if self.is_symbol(r, i - 1)
                || self.is_symbol(r - 1, i - 1)
                || self.is_symbol(r - 1, i)
                || self.is_symbol(r - 1, i + 1)
                || self.is_symbol(r, i + 1)
                || self.is_symbol(r + 1, i + 1)
                || self.is_symbol(r + 1, i)
                || self.is_symbol(r + 1, i - 1)
            {
                return true;
            }
        }

        false
    }

    pub fn part_1(&self) -> usize {
        // Set of (number, (row, col))
        let mut numbers: HashSet<(usize, (usize, usize))> = HashSet::new();
        // For each row
        for r in 0..self.rows {
            // For each column
            for c in 0..self.cols {
                if self.grid[r][c].is_digit(10) {
                    let number = self.get_number_at(r, c);
                    if let Some((num, pos)) = number {
                        numbers.insert((num, pos));
                    }
                }
            }
        }

        // For each entry, check if the number is less than 100
        let valid_numbers: Vec<usize> = numbers
            .iter()
            .filter_map(|(number, position)| {
                if self.is_number_next_to_symbol(position.0, position.1) {
                    Some(*number)
                } else {
                    None
                }
            })
            .collect();
        valid_numbers.iter().sum()
    }
}

pub fn main() -> io::Result<()> {
    let path = Path::new("src/day_3/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    // Read each line and convert it to a Vec<char>
    let input: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<_>>();
    let solution = Solution::new(input);

    println!("Part 1");
    println!("{:?}", solution.part_1());
    Ok(())
}
