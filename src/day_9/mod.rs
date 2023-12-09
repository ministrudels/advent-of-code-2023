use std::io;

struct Solution {
    input: Vec<String>,
}

impl Solution {
    fn new(input: Vec<String>) -> Self {
        Self { input }
    }

    fn compute_next_number(&self, history: &Vec<i64>) -> i64 {
        if history.iter().all(|h| *h == 0) {
            return 0;
        }
        let diffs: Vec<i64> = history
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > 0)
            .map(|(i, h)| h - history[i - 1])
            .collect();
        history[history.len() - 1] + self.compute_next_number(&diffs)
    }

    fn compute_previous_number(&self, history: &Vec<i64>) -> i64 {
        if history.iter().all(|h| *h == 0) {
            return 0;
        }
        let diffs: Vec<i64> = history
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > 0)
            .map(|(i, h)| h - history[i - 1])
            .collect();
        history[0] - self.compute_previous_number(&diffs)
    }

    pub fn part_1(&self) -> i64 {
        let histories: Vec<Vec<i64>> = self
            .input
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        histories
            .iter()
            .map(|history| self.compute_next_number(history))
            .sum()
    }
    pub fn part_2(&self) -> i64 {
        let histories: Vec<Vec<i64>> = self
            .input
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        histories
            .iter()
            .map(|history| self.compute_previous_number(history))
            .sum()
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
        let input = read_input_file("src/day_9/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_1(), 114);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_file("src/day_9/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_2(), 2);
    }
}
