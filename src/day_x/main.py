from typing import List

with open("src/day_x/input.txt", "r") as f:
    input = f.read().split("\n")


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input

    def part_1(self) -> int:
        return 0

    def part_2(self) -> int:
        return 0


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
