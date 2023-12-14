from typing import List

with open("src/day_14/input.txt", "r") as f:
    input = f.read().split("\n")


def shift_rocks(line: List[str]) -> List[str]:
    for i in range(len(line)):
        if line[i] == "O":
            j = i - 1
            while j >= 0 and line[j] != "#":
                line[j], line[j + 1] = line[j + 1], line[j]
                j -= 1
    return line


def transpose(matrix: List[List[str]]) -> List[List[str]]:
    # Assuming the matrix is a list of lists
    rows, cols = len(matrix), len(matrix[0])

    # Use nested list comprehension to transpose the matrix
    transposed = [[matrix[j][i] for j in range(rows)] for i in range(cols)]
    return transposed


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.platform = input
        self.rows = len(input)
        self.cols = len(input[0])
        self.tilted_northward = self.tilted_north()

    def tilted_north(self) -> List[List[str]]:
        tilted = []
        for c in range(self.cols):
            column = []
            for r in range(self.rows):
                column.append(self.platform[r][c])
            tilted.append(shift_rocks(column))
        return transpose(tilted)

    def part_1(self) -> int:
        result = 0
        for i, row in enumerate(self.tilted_northward):
            count_of_o_rocks = row.count("O")
            result += count_of_o_rocks * (len(self.tilted_northward) - i)
        return result

    def part_2(self) -> int:
        return 0


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
