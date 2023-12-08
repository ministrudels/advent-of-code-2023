use core::time;
use num;
use std::collections::HashMap;
use std::io::{self};
use std::time::Instant;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split("=").collect();
        let name = parts[0].trim().to_string();

        let left_and_right: Vec<&str> = line[line.find("(").unwrap() + 1..line.find(")").unwrap()]
            .split(",")
            .collect();
        let left = left_and_right[0].replace(" ", "");
        let right = left_and_right[1].replace(" ", "");
        Self { name, left, right }
    }
}

struct Solution {
    instructions: String,
    nodes: HashMap<String, Node>,
}

impl Solution {
    fn new(input: Vec<String>) -> Self {
        let instructions = input[0].clone();
        let mut nodes = HashMap::new();
        for i in 2..input.len() {
            let parts: Vec<&str> = input[i].split('=').collect();
            let key = parts[0].trim();
            nodes.insert(String::from(key), Node::new(&input[i]));
        }

        Solution {
            instructions,
            nodes,
        }
    }

    fn compute_steps_to_z(&self, node: &Node) -> i64 {
        let mut p = node;
        let mut steps = 0;
        loop {
            for instruction in self.instructions.chars() {
                if p.name.ends_with("Z") {
                    return steps;
                }
                if instruction == 'L' {
                    p = self.nodes.get(&p.left).unwrap();
                } else if instruction == 'R' {
                    p = self.nodes.get(&p.right).unwrap();
                }
                steps += 1;
            }
        }
    }

    fn part_1(&self) -> i32 {
        let mut p = self.nodes.get("AAA").unwrap();
        let mut steps = 0;
        loop {
            for instruction in self.instructions.chars() {
                if p.name.ends_with("ZZZ") {
                    return steps;
                }
                if instruction == 'L' {
                    p = self.nodes.get(&p.left).unwrap();
                } else if instruction == 'R' {
                    p = self.nodes.get(&p.right).unwrap();
                }
                steps += 1;
            }
        }
    }

    fn part_2(&self) -> i64 {
        let starting_nodes: Vec<&Node> = self
            .nodes
            .iter()
            .filter(|(_, node)| node.name.ends_with("A"))
            .map(|(_, node)| node)
            .collect();
        let steps: Vec<i64> = starting_nodes
            .iter()
            .map(|node| self.compute_steps_to_z(node))
            .collect();
        let lcm = steps.iter().fold(1, |acc, &x| num::integer::lcm(acc, x));
        return lcm;
    }
}

pub fn main(input: Vec<String>) -> io::Result<()> {
    fn timed<F: FnMut() -> R, R>(label: &str, mut f: F) -> R {
        let now = Instant::now();
        let ret = f();
        let elapsed = now.elapsed();
        println!("{} took: {:.2?} seconds to run", label, elapsed);
        ret
    }

    let solution = Solution::new(input);
    timed("Part 1", || println!("{:?}", solution.part_1()));
    timed("Part 2", || println!("{:?}", solution.part_2()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input_file("src/day_8/input_sample_p1.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_1(), 2);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_file("src/day_8/input_sample_p2.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_2(), 6);
    }
}
