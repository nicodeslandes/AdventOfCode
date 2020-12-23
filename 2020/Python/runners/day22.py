from typing import List, Set, Tuple

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


def play_single_round(deck1, deck2):
    while any(deck1) and any(deck2):
        c1 = deck1.pop(0)
        c2 = deck2.pop(0)
        if c1 > c2:
            deck1.append(c1)
            deck1.append(c2)
        else:
            deck2.append(c2)
            deck2.append(c1)

    return (1, deck1) if any(deck1) else (2, deck2)


def part1(input: List[str]) -> int:
    deck1, deck2 = parse_decks(input)

    _, winner_deck = play_single_round(deck1, deck2)
    return sum((i+1)*winner_deck[len(winner_deck)-i-1] for i in range(len(winner_deck)))


def play_round(deck1: Deck, deck2: Deck) -> Tuple[int, Deck]:
    previous_rounds: Set[Tuple[Tuple[int], Tuple[int]]] = set()
    while any(deck1) and any(deck2):
        state = tuple(deck1), tuple(deck2)
        if state in previous_rounds:
            return 1, deck1
        else:
            previous_rounds.add(state)

        c1 = deck1.pop(0)
        c2 = deck2.pop(0)
        if c1 > len(deck1) or c2 > len(deck2):
            # Not enough cards for recursive game, play normal
            if c1 > c2:
                deck1.append(c1)
                deck1.append(c2)
            else:
                deck2.append(c2)
                deck2.append(c1)
        else:
            # Recursive game!
            winner, _ = play_round(list(deck1[0:c1]), list(deck2[0:c2]))
            if winner == 1:
                deck1.append(c1)
                deck1.append(c2)
            else:
                deck2.append(c2)
                deck2.append(c1)

    return (1, deck1) if any(deck1) else (2, deck2)


def part2(input: List[str]) -> int:
    deck1, deck2 = parse_decks(input)
    _, winner_deck = play_round(deck1, deck2)
    return sum((i+1)*winner_deck[len(winner_deck)-i-1] for i in range(len(winner_deck)))
