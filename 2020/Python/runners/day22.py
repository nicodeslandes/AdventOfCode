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
    deck1.reverse()
    deck2.reverse()

    while any(deck1) and any(deck2):
        c1 = deck1.pop()
        c2 = deck2.pop()
        if c1 > c2:
            deck1.insert(0, c1)
            deck1.insert(0, c2)
        else:
            deck2.insert(0, c2)
            deck2.insert(0, c1)

    winner_deck = deck1 if any(deck1) else deck2
    return sum((i+1)*winner_deck[i] for i in range(len(winner_deck)))

# def part2(input: List[str]) -> int:
