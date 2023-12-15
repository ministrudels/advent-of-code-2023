use std::{collections::HashMap, io};

fn compute_hash_value(input: &str) -> u32 {
    let mut val: u32 = 0;
    for c in input.chars() {
        let ascii_val: u32 = c.into();
        val += ascii_val;
        val *= 17;
        val %= 256;
    }
    val
}

#[derive(Debug)]
struct Lens {
    label: String,
    power: u32,
}

impl Lens {
    fn new(label: String, power: u32) -> Self {
        Self { label, power }
    }
}

#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn add_lens(&mut self, new_lens: Lens) {
        for lens in &mut self.lenses {
            if lens.label == new_lens.label {
                lens.power = new_lens.power;
                return;
            }
        }
        self.lenses.push(new_lens);
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }

    fn compute_power(&self) -> u32 {
        let mut result = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            result += (i as u32 + 1) * lens.power;
        }
        result
    }
}

struct Solution {
    input: Vec<String>,
}

impl Solution {
    fn new(input: Vec<String>) -> Self {
        let sequence = input[0]
            .split(",")
            .map(|x| x.parse::<String>().unwrap())
            .collect();
        Self { input: sequence }
    }

    pub fn part_1(&self) -> u32 {
        self.input.iter().map(|x| compute_hash_value(x)).sum()
    }
    pub fn part_2(&self) -> u32 {
        let mut boxes: HashMap<u32, Box> = HashMap::new();
        for seq in &self.input {
            let box_label: String = seq.chars().filter(|c| c.is_alphabetic()).collect();
            let box_id: u32 = compute_hash_value(&box_label);
            if seq.contains("-") {
                let label: String = seq.chars().filter(|c| c.is_alphabetic()).collect();
                boxes.get_mut(&box_id).map(|b| b.remove_lens(&label));
            }
            if seq.contains("=") {
                let lens_parts: Vec<&str> = seq.split("=").collect();
                let lens = Lens::new(
                    lens_parts[0].to_string(),
                    lens_parts[1].parse::<u32>().unwrap(),
                );
                boxes.entry(box_id).or_insert(Box::new()).add_lens(lens);
            }
        }

        boxes
            .into_iter()
            .map(|(index, b)| (index + 1) * b.compute_power())
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
        let input = read_input_file("src/day_15/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_1(), 1320);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_file("src/day_15/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_2(), 145);
    }
}
