from typing import List

with open("src/day_9/input.txt", "r") as f:
    input = f.read().split("\n")


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input

    def _compute_next_number(self, history: List[int]) -> int:
        if all([h == 0 for h in history]):
            return 0
        diffs = [history[i] - history[i - 1] for i in range(1, len(history))]
        return history[-1] + self._compute_next_number(diffs)

    def _compute_previous_number(self, history: List[int]) -> int:
        if all([h == 0 for h in history]):
            return 0
        diffs = [history[i] - history[i - 1] for i in range(1, len(history))]
        return history[0] - self._compute_previous_number(diffs)

    def part_1(self) -> int:
        histories: List[List[int]] = [
            list(map(int, line.split())) for line in self.input
        ]
        return sum(list(map(self._compute_next_number, histories)))

    def part_2(self) -> int:
        histories: List[List[int]] = [
            list(map(int, line.split())) for line in self.input
        ]
        return sum(list(map(self._compute_previous_number, histories)))


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
