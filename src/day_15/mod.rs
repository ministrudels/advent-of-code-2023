use std::io;

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

pub fn main(input: Vec<String>) -> io::Result<()> {
    let solution = Solution::new(input);

    println!("Part 1");
    println!("{:?}", solution.part_1());
    println!("Part 2");
    println!("{:?}", solution.part_2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input_file("src/day_x/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_1(), 0);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_file("src/day_x/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_2(), 0);
    }
}
