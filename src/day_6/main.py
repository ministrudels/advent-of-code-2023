from typing import List

with open("src/day_6/input.txt", "r") as f:
    input = f.read().split("\n")


class Race:
    def __init__(self, time: int, record: int) -> None:
        self.time = time
        self.record = record

    def get_ways_to_win_race(self) -> int:
        """
        The distinct number of speeds or seconds to hold the button down to beat the record
        """
        min_speed = 0
        max_speed = self.time

        while min_speed * (self.time - min_speed) <= self.record:
            min_speed += 1

        while max_speed * (self.time - max_speed) <= self.record:
            max_speed -= 1

        return max_speed - min_speed + 1


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input

    def part_1(self) -> int:
        times = list(map(int, self.input[0].split(":")[1].split()))
        records = list(map(int, self.input[1].split(":")[1].split()))
        product = 1
        for t, r in zip(times, records):
            race = Race(t, r)
            product *= race.get_ways_to_win_race()
        return product

    def part_2(self) -> int:
        time = int(self.input[0].split(":")[1].replace(" ", ""))
        record = int(self.input[1].split(":")[1].replace(" ", ""))
        race = Race(time, record)
        return race.get_ways_to_win_race()


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
