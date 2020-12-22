from typing import List, Tuple

Deck = List[int]


def parse_decks(input: List[str]) -> Tuple[Deck, Deck]:
    deck1 = []
    deck2 = []
    current_deck = deck1
    for line in input[1:]:
        if line == "" or line.startswith("Player"):
            current_deck = deck2
        else:
            current_deck.append(int(line))
    return deck1, deck2


def part1(input: List[str]) -> int:
    deck1, deck2 = parse_decks(input)

    while any(deck1) and any(deck2):
        c1 = deck1.pop(0)
        c2 = deck2.pop(0)
        if c1 > c2:
            deck1.append(c1)
            deck1.append(c2)
        else:
            deck2.append(c2)
            deck2.append(c1)

    winner_deck = deck1 if any(deck1) else deck2
    return sum((i+1)*winner_deck[len(winner_deck)-i-1] for i in range(len(winner_deck)))


def part2(input: List[str]) -> int:
    return 0
