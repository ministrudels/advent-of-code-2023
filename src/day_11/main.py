from typing import List, Tuple
from itertools import combinations
import numpy as np


def read_file_as_2d_char_array(filename):
    with open(filename, "r") as file:
        lines = file.readlines()
    return [list(line.strip()) for line in lines]


class Solution:
    def __init__(self, input: List[List[str]]) -> None:
        self.input = input
        self.empty_rows = [
            row for row in range(len(self.input)) if self.is_empty_row(row)
        ]
        self.empty_cols = [
            col for col in range(len(self.input[0])) if self.is_empty_col(col)
        ]

    def is_empty_row(self, row: int) -> bool:
        return all(cell == "." for cell in self.input[row])

    def is_empty_col(self, col: int) -> bool:
        return all(row[col] == "." for row in self.input)

    def shortest_path(
        self, start: Tuple[int, int], end: Tuple[int, int], factor
    ) -> int:
        empty_cols_between = len(
            [
                col
                for col in self.empty_cols
                if end[1] < col < start[1] or start[1] < col < end[1]
            ]
        )
        empty_rows_between = len(
            [
                row
                for row in self.empty_rows
                if end[0] < row < start[0] or start[0] < row < end[0]
            ]
        )
        raw_x = abs(start[1] - end[1])
        raw_y = abs(start[0] - end[0])
        x_distance = raw_x - empty_cols_between + empty_cols_between * factor
        y_distance = raw_y - empty_rows_between + empty_rows_between * factor
        return x_distance + y_distance

    def calculate_sum_of_paths_between_all_galaxies(self, factor: int) -> int:
        galaxies = []
        for row in range(len(self.input)):
            for col in range(len(self.input[row])):
                if self.input[row][col] == "#":
                    galaxies.append((row, col))
        pairs = list(combinations(galaxies, 2))
        result = 0

        for pair in pairs:
            result += self.shortest_path(pair[0], pair[1], factor)

        return result

    def part_1(self) -> int:
        return self.calculate_sum_of_paths_between_all_galaxies(2)

    def part_2(self) -> int:
        return self.calculate_sum_of_paths_between_all_galaxies(1000000)


solution = Solution(read_file_as_2d_char_array("src/day_11/input.txt"))
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
