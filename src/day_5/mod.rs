use std::{io, time::Instant};

#[derive(Debug)]
struct Mapper {
    ranges: Vec<(i64, i64, i64)>,
}

impl Mapper {
    fn new(match_str: &str, input_data: &Vec<String>) -> Mapper {
        let mut ranges = Vec::new();
        let mut found = false;

        for line in input_data {
            if line.contains(match_str) {
                found = true;
            } else if found && !line.trim().is_empty() {
                let numbers: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                ranges.push((numbers[0], numbers[1], numbers[2]));
            } else {
                found = false;
            }
        }

        Mapper { ranges }
    }

    fn convert(&self, val: i64) -> i64 {
        for (destination, source, range_val) in &self.ranges {
            if *source <= val && val < *source + *range_val {
                return *destination + (val - *source);
            }
        }
        val
    }

    fn convert_from(&self, val: i64) -> i64 {
        for (destination, source, range_val) in &self.ranges {
            if *destination <= val && val < *destination + *range_val {
                return *source + (val - *destination);
            }
        }
        val
    }
}

struct Solution {
    input_data: Vec<String>,
    seed_to_soil: Mapper,
    soil_to_fertilizer: Mapper,
    fertilizer_to_water: Mapper,
    water_to_light: Mapper,
    light_to_temperature: Mapper,
    temperature_to_humidity: Mapper,
    humidity_to_location: Mapper,
}

impl Solution {
    fn new(input_data: Vec<String>) -> Solution {
        let seed_to_soil = Mapper::new("seed-to-soil map", &input_data);
        let soil_to_fertilizer = Mapper::new("soil-to-fertilizer map", &input_data);
        let fertilizer_to_water = Mapper::new("fertilizer-to-water map", &input_data);
        let water_to_light = Mapper::new("water-to-light map", &input_data);
        let light_to_temperature = Mapper::new("light-to-temperature map", &input_data);
        let temperature_to_humidity = Mapper::new("temperature-to-humidity map", &input_data);
        let humidity_to_location = Mapper::new("humidity-to-location map", &input_data);

        Solution {
            input_data,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn seed_to_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.convert(seed);
        let fertilizer = self.soil_to_fertilizer.convert(soil);
        let water = self.fertilizer_to_water.convert(fertilizer);
        let light = self.water_to_light.convert(water);
        let temperature = self.light_to_temperature.convert(light);
        let humidity = self.temperature_to_humidity.convert(temperature);
        self.humidity_to_location.convert(humidity)
    }

    fn location_to_seed(&self, location: i64) -> i64 {
        let humidity = self.humidity_to_location.convert_from(location);
        let temperature = self.temperature_to_humidity.convert_from(humidity);
        let light = self.light_to_temperature.convert_from(temperature);
        let water = self.water_to_light.convert_from(light);
        let fertilizer = self.fertilizer_to_water.convert_from(water);
        let soil = self.soil_to_fertilizer.convert_from(fertilizer);
        self.seed_to_soil.convert_from(soil)
    }

    fn part_1(&self) -> i64 {
        let mut locations = Vec::new();
        let seeds: Vec<i64> = self.input_data[0].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for seed in seeds {
            locations.push(self.seed_to_location(seed));
        }

        *locations.iter().min().unwrap()
    }

    fn part_2(&self) -> i64 {
        let seed_input: Vec<i64> = self.input_data[0].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut seed_ranges = Vec::new();

        for i in (0..seed_input.len()).step_by(2) {
            seed_ranges.push(vec![seed_input[i], seed_input[i] + seed_input[i + 1]]);
        }

        for i in 0..100000000 {
            for r in &seed_ranges {
                if r[0] <= self.location_to_seed(i) && self.location_to_seed(i) < r[1] {
                    return i;
                }
            }
        }
        0
    }
}

pub fn main(input: Vec<String>) -> io::Result<()> {
    let solution = Solution::new(input);

    println!("Part 1");
    println!("{:?}", solution.part_1());
    println!("Part 2");
    let now = Instant::now();
    println!("{:?}", solution.part_2());
    let elapsed = now.elapsed();
    println!("My function took: {:.2?} seconds to run", elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input_file("src/day_5/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_1(), 35);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_file("src/day_5/input_sample.txt").unwrap();
        let solution = Solution::new(input);
        assert_eq!(solution.part_2(), 46);
    }
}
