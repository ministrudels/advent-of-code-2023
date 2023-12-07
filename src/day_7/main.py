from collections import defaultdict
from functools import cmp_to_key
from typing import List
from enum import Enum

with open("src/day_7/input.txt", "r") as f:
    input = f.read().split("\n")


class HandType(Enum):
    FIVEOFAKIND = 7
    FOUROFAKIND = 6
    FULLHOUSE = 5
    THREEOFAKIND = 4
    TWOPAIR = 3
    ONEPAIR = 2
    HIGHCARD = 1


class Hand:
    def __init__(self, line: str):
        self.cards = line.split(" ")[0]
        self.bet = line.split(" ")[1]

    def get_hand_type(self) -> HandType:
        cards: defaultdict[str, int] = defaultdict(int)
        for card in self.cards:
            cards[card] += 1
        # parse default dict as a list sorted by value
        sorted_cards = sorted(cards.items(), key=lambda x: x[1], reverse=True)
        if sorted_cards[0][1] == 5:
            return HandType.FIVEOFAKIND
        elif sorted_cards[0][1] == 4:
            return HandType.FOUROFAKIND
        elif sorted_cards[0][1] == 3 and sorted_cards[1][1] == 2:
            return HandType.FULLHOUSE
        elif sorted_cards[0][1] == 3:
            return HandType.THREEOFAKIND
        elif sorted_cards[0][1] == 2 and sorted_cards[1][1] == 2:
            return HandType.TWOPAIR
        elif sorted_cards[0][1] == 2:
            return HandType.ONEPAIR
        else:
            return HandType.HIGHCARD

    def get_hand_type_with_jokers(self) -> HandType:
        cards: defaultdict[str, int] = defaultdict(int)
        jokers = 0
        for card in self.cards:
            if card == "J":
                jokers += 1
                continue
            cards[card] += 1
        sorted_cards = sorted(cards.items(), key=lambda x: x[1], reverse=True)
        # Edge case if we have 5 jokers
        if jokers == 5:
            return HandType.FIVEOFAKIND
        highest_card = sorted_cards[0][1] + jokers
        if highest_card == 5:
            return HandType.FIVEOFAKIND
        elif highest_card == 4:
            return HandType.FOUROFAKIND
        elif highest_card == 3 and sorted_cards[1][1] == 2:
            return HandType.FULLHOUSE
        elif highest_card == 3:
            return HandType.THREEOFAKIND
        elif highest_card == 2 and sorted_cards[1][1] == 2:
            return HandType.TWOPAIR
        elif highest_card == 2:
            return HandType.ONEPAIR
        else:
            return HandType.HIGHCARD


class Solution:
    def __init__(self, input: List[str]) -> None:
        self.input = input

    def compare_hands(self, hand1: Hand, hand2: Hand) -> int:
        """
        Compare hand values in ascending order
        """
        card_ranking = {
            "A": 14,
            "K": 13,
            "Q": 12,
            "J": 11,
            "T": 10,
            "9": 9,
            "8": 8,
            "7": 7,
            "6": 6,
            "5": 5,
            "4": 4,
            "3": 3,
            "2": 2,
        }
        hand1_type = hand1.get_hand_type()
        hand2_type = hand2.get_hand_type()
        if hand1_type.value != hand2_type.value:
            return hand1_type.value - hand2_type.value
        i = 0
        while i < 5:
            if card_ranking[hand1.cards[i]] != card_ranking[hand2.cards[i]]:
                return card_ranking[hand1.cards[i]] - card_ranking[hand2.cards[i]]
            i += 1

    def compare_hands_with_jokers(self, hand1: Hand, hand2: Hand) -> int:
        """
        Compare hand values in ascending order
        """
        card_ranking = {
            "A": 14,
            "K": 13,
            "Q": 12,
            "J": 1,
            "T": 10,
            "9": 9,
            "8": 8,
            "7": 7,
            "6": 6,
            "5": 5,
            "4": 4,
            "3": 3,
            "2": 2,
        }
        hand1_type = hand1.get_hand_type_with_jokers()
        hand2_type = hand2.get_hand_type_with_jokers()
        if hand1_type.value != hand2_type.value:
            return hand1_type.value - hand2_type.value
        # We order of cards
        i = 0
        while i < 5:
            if card_ranking[hand1.cards[i]] != card_ranking[hand2.cards[i]]:
                return card_ranking[hand1.cards[i]] - card_ranking[hand2.cards[i]]
            i += 1

    def part_1(self) -> int:
        hands = [Hand(line) for line in self.input]
        ranked_hands = sorted(
            hands,
            key=cmp_to_key(self.compare_hands),
        )
        total_winnings = 0
        for rank, hand in enumerate(ranked_hands):
            total_winnings += (rank + 1) * int(hand.bet)
        return total_winnings

    def part_2(self) -> int:
        hands = [Hand(line) for line in self.input]
        ranked_hands = sorted(
            hands,
            key=cmp_to_key(self.compare_hands_with_jokers),
        )
        total_winnings = 0
        for rank, hand in enumerate(ranked_hands):
            total_winnings += (rank + 1) * int(hand.bet)
        return total_winnings


solution = Solution(input)
print("Part 1")
print(solution.part_1())
print("Part 2")
print(solution.part_2())
