from typing import Dict, Tuple, Set, List

with open("src/day_3/input.txt", "r") as f:
    input = f.read().split("\n")

# Pad the input with "." at the end since our logic relies on doing something when we see a change
input = [line + "." for line in input]

ROWS = len(input)
COLS = len(input[0])


def is_symbol(r: int, c: int) -> bool:
    """
    Check if the current position is a symbol
    """
    if r < 0 or r >= ROWS or c < 0 or c >= COLS:
        return False
    x = input[r][c]
    return x is not None and x != "." and not x.isdigit()


def is_adjacent_to_symbol(r: int, c: int) -> bool:
    """
    Check if the current position is adjacent to a symbol
    """

    return (
        is_symbol(r, c - 1)
        or is_symbol(r - 1, c - 1)
        or is_symbol(r - 1, c)
        or is_symbol(r - 1, c + 1)
        or is_symbol(r, c + 1)
        or is_symbol(r + 1, c + 1)
        or is_symbol(r + 1, c)
        or is_symbol(r + 1, c - 1)
    )


def get_number_at(r: int, c: int) -> Tuple[int, Tuple[int, int]]:
    """
    For a digit, get the whole digit, and the starting position of the digit
    """
    if r < 0 or r >= ROWS or c < 0 or c >= COLS:
        return None
    p = c
    while input[r][p].isdigit():
        p -= 1
    p += 1
    starting_position = (r, p)
    result = ""
    while input[r][p].isdigit():
        result = result + input[r][p]
        p += 1
    return int(result), starting_position


def part_1() -> int:
    parts_sum = 0

    for r in range(ROWS):
        is_number_adjacent_to_symbol = False
        number = ""
        for c in range(COLS):
            if input[r][c].isdigit():
                number = number + input[r][c]
                is_number_adjacent_to_symbol = (
                    is_number_adjacent_to_symbol or is_adjacent_to_symbol(r, c)
                )
            else:
                if is_number_adjacent_to_symbol:
                    parts_sum += int(number)
                number = ""
                is_number_adjacent_to_symbol = False

    return parts_sum


def part_2() -> int:
    gears: Set[Tuple[int, int]] = set()
    result = 0

    # Find locations of all the gears
    for r in range(ROWS):
        for c in range(COLS):
            if input[r][c] == "*":
                gears.add((r, c))

    # For every gear find the adjacent numbers
    for g in gears:
        teeth: Dict[Tuple[int, int], int] = {}
        if input[g[0]][g[1] - 1].isdigit():
            number, starting_position = get_number_at(g[0], g[1] - 1)
            teeth[starting_position] = number
        if input[g[0] - 1][g[1] - 1].isdigit():
            number, starting_position = get_number_at(g[0] - 1, g[1] - 1)
            teeth[starting_position] = number
        if input[g[0] - 1][g[1]].isdigit():
            number, starting_position = get_number_at(g[0] - 1, g[1])
            teeth[starting_position] = number
        if input[g[0] - 1][g[1] + 1].isdigit():
            number, starting_position = get_number_at(g[0] - 1, g[1] + 1)
            teeth[starting_position] = number
        if input[g[0]][g[1] + 1].isdigit():
            number, starting_position = get_number_at(g[0], g[1] + 1)
            teeth[starting_position] = number
        if input[g[0] + 1][g[1] + 1].isdigit():
            number, starting_position = get_number_at(g[0] + 1, g[1] + 1)
            teeth[starting_position] = number
        if input[g[0] + 1][g[1]].isdigit():
            number, starting_position = get_number_at(g[0] + 1, g[1])
            teeth[starting_position] = number
        if input[g[0] + 1][g[1] - 1].isdigit():
            number, starting_position = get_number_at(g[0] + 1, g[1] - 1)
            teeth[starting_position] = number
        # Check for all the gears which have exactly 2 numbers adjacent to them
        if len(teeth) == 2:
            # Multiply all products
            gear_ratio = 1
            for key in teeth:
                gear_ratio *= teeth[key]
            result += gear_ratio
    return result


print("Part 1")
print(part_1())

print("Part 2")
print(part_2())


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input
        self.rows = len(input)
        self.cols = len(input[0])
        pass
