import time
from typing import List, Dict
import numpy as np

with open("src/day_8/input.txt", "r") as f:
    input = f.read().split("\n")


class Node:
    def __init__(self, line: str) -> None:
        self.name = line.split("=")[0].strip()
        left_and_right = line[line.index("(") + 1 : line.index(")")]
        self.left = left_and_right.split(",")[0]
        self.right = left_and_right.split(" ")[1]

    def __repr__(self) -> str:
        return f"{self.name} -> {self.left} | {self.right}"

    def __str__(self) -> str:
        return f"{self.name}"


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input
        self.instructions = self.input[0]
        self.nodes: Dict[str, Node] = {}
        for i in range(2, len(self.input)):
            key = self.input[i].split("=")[0].strip()
            self.nodes[key] = Node(self.input[i])

    def compute_steps_to_z(self, node: Node) -> int:
        p = node
        steps = 0
        while True:
            for instruction in self.instructions:
                if p.name.endswith("Z"):
                    return steps
                if instruction == "L":
                    p = self.nodes[p.left]
                elif instruction == "R":
                    p = self.nodes[p.right]
                steps += 1

    def part_1(self) -> int:
        p = self.nodes["AAA"]
        steps = 0
        while True:
            for instruction in self.instructions:
                if p == self.nodes["ZZZ"]:
                    return steps
                if instruction == "L":
                    p = self.nodes[p.left]
                elif instruction == "R":
                    p = self.nodes[p.right]
                steps += 1

    def part_2(self) -> int:
        # Get all nodes where key ends with A
        pointers = [self.nodes[n] for n in self.nodes if n.endswith("A")]
        steps = []

        for p in pointers:
            steps.append(self.compute_steps_to_z(p))
        # lowest common multiple of all steps
        return np.lcm.reduce(steps)


solution = Solution(input)


def timed(label: str, func: callable):
    start_time = time.time()
    result = func()
    end_time = time.time()
    print(f"{label}: {result} took {1000 * (end_time - start_time)}ms")


timed("Part 1", solution.part_1)
timed("Part 2", solution.part_2)
