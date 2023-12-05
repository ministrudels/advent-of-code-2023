from typing import Tuple, List
import time

with open("src/day_5/input.txt", "r") as f:
    input = f.read().split("\n")


class Mapper:
    def __init__(self, match: str, input: List[str]) -> None:
        self.ranges: List[Tuple[int, int, int]] = []
        found = False

        for line in input:
            if match in line:
                found = True
            elif found and line.strip():  # Check if the line is not empty
                numbers = tuple(map(int, line.split()))
                self.ranges.append(numbers)
            else:
                found = False

    def __repr__(self) -> str:
        return f"Destination: {self.destination}, Source: {self.source}, Range: {self.range}"

    def __str__(self) -> str:
        return f"Destination: {self.destination}, Source: {self.source}, Range: {self.range}"

    def convert(self, val: int) -> int:
        for destination, source, range in self.ranges:
            if source <= val < source + range:
                return destination + (val - source)
        return val

    def convert_from(self, val: int) -> int:
        for destination, source, range in self.ranges:
            if destination <= val < destination + range:
                return source + (val - destination)
        return val


# destination range start, the source range start, and the range length.
class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input
        self.seed_to_soil = Mapper("seed-to-soil map", input)
        self.soil_to_fertilizer = Mapper("soil-to-fertilizer map", input)
        self.fertilizer_to_water = Mapper("fertilizer-to-water map", input)
        self.water_to_light = Mapper("water-to-light map", input)
        self.light_to_temperature = Mapper("light-to-temperature map", input)
        self.temperature_to_humidity = Mapper("temperature-to-humidity map", input)
        self.humidity_to_location = Mapper("humidity-to-location map", input)

    def seed_to_location(self, seed: int) -> int:
        soil = self.seed_to_soil.convert(seed)
        fertilizer = self.soil_to_fertilizer.convert(soil)
        water = self.fertilizer_to_water.convert(fertilizer)
        light = self.water_to_light.convert(water)
        temperature = self.light_to_temperature.convert(light)
        humidity = self.temperature_to_humidity.convert(temperature)
        location = self.humidity_to_location.convert(humidity)
        return location

    def location_to_seed(self, location: int) -> int:
        humidity = self.humidity_to_location.convert_from(location)
        temperature = self.temperature_to_humidity.convert_from(humidity)
        light = self.light_to_temperature.convert_from(temperature)
        water = self.water_to_light.convert_from(light)
        fertilizer = self.fertilizer_to_water.convert_from(water)
        soil = self.soil_to_fertilizer.convert_from(fertilizer)
        seed = self.seed_to_soil.convert_from(soil)
        return seed

    def part_1(self) -> int:
        locations = []
        seeds = list(map(int, input[0].split(":")[1].split()))
        for seed in seeds:
            locations.append(self.seed_to_location(seed))
        return min(locations)

    def part_2(self) -> int:
        seed_input = list(map(int, input[0].split(":")[1].split()))
        seed_ranges = [
            [seed_input[i], seed_input[i] + seed_input[i + 1]]
            for i in range(0, len(seed_input), 2)
        ]
        # From location 0 to 100000000, check if it maps to a seed, if it does thats the lowest location
        for i in range(100000000):
            for r in seed_ranges:
                if r[0] <= self.location_to_seed(i) < r[1]:
                    return i


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
start_time = time.time()
print(solution.part_2())
end_time = time.time()
print(f"My function took {end_time - start_time:.6f} seconds to run.")
