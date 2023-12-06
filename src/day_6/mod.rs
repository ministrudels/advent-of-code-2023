use std::io;

struct Race {
    time: u64,
    record_distance: u64,
}
impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self {
            time,
            record_distance: distance,
        }
    }
    fn get_ways_to_win_race(&self) -> u64 {
        let mut min_speed = 0;
        let mut max_speed = self.time;
        while min_speed * (self.time - min_speed) <= self.record_distance {
            min_speed += 1;
        }
        while max_speed * (self.time - max_speed) <= self.record_distance {
            max_speed -= 1;
        }
        return max_speed - min_speed + 1;
    }
}
#[allow(dead_code)]
struct Solution {
    input: Vec<String>,
}

impl Solution {
    fn new(input: Vec<String>) -> Self {
        Self { input }
    }

    pub fn part_1(&self) -> u64 {
        let times: Vec<u64> = self.input[0].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let records: Vec<u64> = self.input[1].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let races = times
            .into_iter()
            .zip(records.into_iter())
            .map(|(t, r)| Race::new(t, r))
            .collect::<Vec<_>>();
        let mut product = 1;
        for race in races {
            product *= race.get_ways_to_win_race();
        }
        product
    }
    pub fn part_2(&self) -> u64 {
        let time = self.input[0].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let record = self.input[1].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let race = Race::new(time, record);
        race.get_ways_to_win_race()
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
        let input = read_input_file("src/day_6/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_1(), 288);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_file("src/day_6/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_2(), 71503);
    }
}
