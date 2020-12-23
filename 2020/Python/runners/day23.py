from typing import List
from logging import debug

Cups = List[int]


def next(cups: Cups) -> Cups:
    minc = min(cups)
    maxc = max(cups)
    c = cups[0]
    moved = list(cups.pop(1) for i in range(3))
    destination = c - 1
    while destination not in cups:
        destination -= 1
        if destination < minc:
            destination = maxc

    dest_index = cups.index(destination)
    for cup in reversed(moved):
        cups.insert(dest_index + 1, cup)

    cups.pop(0)
    cups.append(c)

    return cups


def part1(input: List[str]) -> str:
    cups = list(map(int, input[0]))
    for i in range(100):
        debug("Cups: %s", cups)
        cups = next(cups)

    index = cups.index(1)

    return "".join(map(str, cups[index+1:] + cups[:index]))

# def part2(input: List[str]) -> int:
