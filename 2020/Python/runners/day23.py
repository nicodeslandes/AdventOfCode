import logging
from runners.utils import isEnabled
from typing import List, Optional, Tuple
from logging import debug, info

Cups = List[int]


def print_cups(cup: int, cups: Cups, n=None) -> str:
    c = cup
    labels = []
    for i in range(5 if not n else n):
        labels.append(c + 1)
        c = cups[c]
    return str(labels)


def next(cup: int, cups: Cups) -> int:
    removed = []
    ri = cups[cup]
    for i in range(3):
        removed.append(ri)
        ri = cups[ri]

    destination = cup
    while True:
        destination -= 1
        if destination < 0:
            destination = len(cups) - 1

        if not destination in removed:
            break

    debug("Destination cup: %d", destination)

    # Insert removed cups
    tmp = cups[destination]
    cups[destination] = removed[0]
    cups[removed[-1]] = tmp
    cups[cup] = ri

    # debug("After dest cup %d: %s", destination, print_cups(dest_cup))

    return ri


def play_rounds(cup: int, cups: Cups, iterations: int) -> Tuple[int, Cups]:
    for i in range(iterations):
        # if i % 100_000 == 0:
        #     print(".", end='')
        if isEnabled(logging.DEBUG):
            debug("Cups %d: %s", i+1, print_cups(cup, cups, 10))
        cup = next(cup, cups)
    return cup, cups


def parse_cups(input: str) -> Cups:
    cups = [0 for i in input]
    p = None
    for c in input:
        i = int(c) - 1
        if p is not None:
            cups[p] = i
        p = i

    cups[p] = int(input[0]) - 1
    return cups


def part1(input: List[str]) -> str:
    cups = parse_cups(input[0])
    debug("Cups: %s", cups)
    start = int(input[0][0]) - 1
    _, cups = play_rounds(start, cups, 100)

    c = 0
    labels = []
    for i in range(1, len(cups)):
        labels.append(str(cups[c] + 1))
        c = cups[c]

    return "".join(labels)


def part2(input: List[str]) -> int:
    cups = parse_cups(input[0])
    start = int(input[0][0]) - 1

    for i in range(len(cups), 1_000_000):
        cups.append(i+1)

    cups[len(cups) - 1] = start
    cups[int(input[0][-1]) - 1] = len(input[0])

    info("Cups: %s", print_cups(start, cups))
    _, cups = play_rounds(start, cups, 10_000_000)

    return (cups[0] + 1) * (cups[cups[0]] + 1)
