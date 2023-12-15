from collections import defaultdict
from typing import List, Dict

with open("src/day_15/input.txt", "r") as f:
    input = f.read().split("\n")


class Lens:
    def __init__(self, label: str, power: str) -> None:
        self.label = label
        self.power = int(power)

    def __repr__(self) -> str:
        return f"Lens({self.label}, {self.power})"


class Box:
    def __init__(self) -> None:
        self.lenses: List[Lens] = []

    def __repr__(self) -> str:
        return f"Box({self.lenses})"

    def add_lens(self, new_lens: Lens):
        for i in range(len(self.lenses)):
            if self.lenses[i].label == new_lens.label:
                self.lenses[i] = new_lens
                return
        self.lenses.append(new_lens)

    def remove_lens(self, label: str):
        for lens in self.lenses:
            if lens.label == label:
                self.lenses.remove(lens)

    def compute_power(self) -> int:
        result = 0
        for i in range(len(self.lenses)):
            result += (i + 1) * self.lenses[i].power
        return result


def compute_hash_value(input: str) -> int:
    val = 0
    for c in input:
        ascii_val = ord(c)
        val += ascii_val
        val *= 17
        val %= 256
    return val


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input_sequence = input[0].split(",")

    def part_1(self) -> int:
        result = 0
        for seq in self.input_sequence:
            result += compute_hash_value(seq)
        return result

    def part_2(self) -> int:
        boxes: Dict[int, Box] = defaultdict(Box)
        for seq in self.input_sequence:
            box_label = "".join([c for c in seq if c.isalpha()])
            box_id = compute_hash_value(box_label)
            if "-" in seq:
                # remove - from seq
                label = "".join([c for c in seq if c.isalpha()])
                boxes[box_id].remove_lens(label)
            if "=" in seq:
                lens = Lens(seq.split("=")[0], seq.split("=")[1])
                boxes[box_id].add_lens(lens)

        result = 0
        for id, box in boxes.items():
            result += (id + 1) * box.compute_power()
        return result


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
