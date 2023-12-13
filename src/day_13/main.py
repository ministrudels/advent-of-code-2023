from typing import List

with open("src/day_13/input.txt", "r") as f:
    input = f.read().split("\n\n")


def find_y_symmetry_index(matrix: List[str]) -> int:
    # i is the upper side of line of  symmetry
    for i in range(0, len(matrix) - 1):
        # check if the line of symmetry is valid
        l = i
        r = i + 1
        is_symmetric = False
        while l >= 0 and r < len(matrix):
            if matrix[l] != matrix[r]:
                is_symmetric = False
                break
            l -= 1
            r += 1
            is_symmetric = True

        if is_symmetric:
            return i
    return -1


def transpose(matrix: List[str]) -> List[str]:
    # Assuming the matrix is a list of lists
    rows, cols = len(matrix), len(matrix[0])

    # Use nested list comprehension to transpose the matrix
    transposed = [[matrix[j][i] for j in range(rows)] for i in range(cols)]
    return ["".join(t) for t in transposed]


class Pattern:
    def __init__(self, pattern: List[str]) -> None:
        self.pattern = pattern
        self.rows, self.cols = len(pattern), len(pattern[0])

    # index of the rhs of the line of symmetry
    def find_x_symmetry_index(self) -> int:
        """
        Symmetrical about the x axis
        """
        return find_y_symmetry_index(self.pattern)

    # index of the rhs of the line of symmetry
    def find_y_symmetry_index(self) -> int:
        """
        Symmetrical about the y axis
        """
        return find_y_symmetry_index(transpose(self.pattern))


class Solution:
    def __init__(self, input: List[str]) -> None:
        pattern_strs = [map_str.strip().split("\n") for map_str in input]
        self.patterns = [Pattern(pattern_str) for pattern_str in pattern_strs]

    def part_1(self) -> int:
        result = 0
        for p in self.patterns:
            horizontal_line_of_reflection = p.find_x_symmetry_index()
            vertical_line_of_reflection = p.find_y_symmetry_index()
            if horizontal_line_of_reflection == -1:
                result += vertical_line_of_reflection + 1
            elif vertical_line_of_reflection == -1:
                result += 100 * (horizontal_line_of_reflection + 1)
        return result

    def part_2(self) -> int:
        return 0


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
