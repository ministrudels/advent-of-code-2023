import re
from typing import List

with open("src/day_2/input.txt", "r") as f:
    input = f.read().split("\n")


def find_red_cubes(line: str) -> List[int]:
    pattern = r"(\d+)\s*red"
    matches = re.findall(pattern, line)
    return [int(x) for x in matches]


def find_green_cubes(line: str) -> List[int]:
    pattern = r"(\d+)\s*green"
    matches = re.findall(pattern, line)
    return [int(x) for x in matches]


def find_blue_cubes(line: str) -> List[int]:
    pattern = r"(\d+)\s*blue"
    matches = re.findall(pattern, line)
    return [int(x) for x in matches]


# Part 1
def part_1() -> int:
    counter = 0
    for line in input:
        game_id = int(re.findall(r"Game\s(\d+)", line)[0])
        reds = find_red_cubes(line)
        greens = find_green_cubes(line)
        blues = find_blue_cubes(line)
        isRedsLessThan = all([x <= 12 for x in reds])
        isGreensLessThan = all([x <= 13 for x in greens])
        isBluesLessThan = all([x <= 14 for x in blues])
        if isRedsLessThan and isGreensLessThan and isBluesLessThan:
            counter += game_id
    return counter


def part_2() -> int:
    counter = 0
    for line in input:
        reds = find_red_cubes(line)
        greens = find_green_cubes(line)
        blues = find_blue_cubes(line)
        power = max(reds) * max(greens) * max(blues)
        counter += power
    return counter


print(part_1())
print(part_2())
