from typing import Dict, List, Set, Tuple

with open("src/day_4/input.txt", "r") as f:
    input = f.read().split("\n")


class Card:
    def __init__(self, card_string):
        # Split the input string into parts
        parts = card_string.split(":")

        # Extract card ID
        self.id = int(parts[0].strip().replace("Card", ""))

        # Extract winning numbers and holding numbers
        numbers = parts[1].split("|")
        self.winning_numbers = list(map(int, numbers[0].strip().split()))
        self.holding_numbers = list(map(int, numbers[1].strip().split()))

    def __str__(self):
        return f"Card {self.id}: Winning Numbers - {self.winning_numbers}, Holding Numbers - {self.holding_numbers}"

    def points_won(self) -> int:
        """
        Returns the number of points won by the holding numbers in the winning numbers.
        """
        matches = len(set(self.winning_numbers).intersection(self.holding_numbers))
        if matches <= 1:
            return matches
        return 2 ** (matches - 1)

    def matches(self) -> bool:
        """
        Returns the number of matches between the winning numbers and holding numbers.
        """
        return len(set(self.winning_numbers).intersection(self.holding_numbers))


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input

    def part_1(self) -> int:
        total_points = 0
        for line in self.input:
            card = Card(line)
            total_points += card.points_won()
        return total_points

    def part_2(self) -> int:
        won_cards = [0] * len(self.input)

        for i, line in enumerate(self.input):
            card = Card(line)
            matches = card.matches()
            won_cards[i] += 1
            for j in range(i + 1, i + matches + 1):
                won_cards[j] += won_cards[i]
        return sum(won_cards)


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
