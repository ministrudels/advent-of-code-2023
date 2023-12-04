use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

struct Card {
    winning_numbers: Vec<i8>,
    holding_numbers: Vec<i8>,
}

impl Card {
    fn new(line: String) -> Self {
        let numbers: &str = line.split(":").collect::<Vec<&str>>()[1];
        let numbers: Vec<&str> = numbers.split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<i8> = numbers[0]
            .split_whitespace()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();
        let holding_numbers: Vec<i8> = numbers[1]
            .split_whitespace()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();
        Self {
            winning_numbers,
            holding_numbers,
        }
    }
    fn points_won(&self) -> u32 {
        let matches: u32 = self
            .winning_numbers
            .iter()
            .filter(|num| self.holding_numbers.contains(num))
            .collect::<Vec<_>>()
            .len()
            .try_into()
            .unwrap();
        if matches < 1 {
            return matches;
        }
        return 2_u32.pow(matches - 1);
    }

    fn matches(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|num| self.holding_numbers.contains(num))
            .collect::<Vec<_>>()
            .len()
    }
}

struct Solution {
    input: Vec<String>,
}

impl Solution {
    fn new(input: Vec<String>) -> Self {
        Self { input }
    }

    pub fn part_1(&self) -> u32 {
        self.input
            .iter()
            .map(|line| Card::new(line.to_string()).points_won())
            .sum()
    }
    pub fn part_2(&self) -> usize {
        let mut won_cards = vec![0; self.input.len()];

        for (i, line) in self.input.iter().enumerate() {
            let card = Card::new(line.to_string());
            let matches = card.matches();
            won_cards[i] += 1;

            for j in (i + 1)..(i + matches + 1).min(self.input.len()) {
                won_cards[j] += won_cards[i];
            }
        }

        won_cards.iter().sum()
    }
}

pub fn main() -> io::Result<()> {
    let path = Path::new("src/day_4/input.txt");
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
