import logging
from runners.utils import isEnabled
from typing import List, Optional, Tuple
from logging import debug

N = 10

Cups = List[int]


def print_cups(cup: int, cups: Cups, n=None) -> str:
    c = cup
    labels = []
    for i in range(5 if not n else n):
        labels.append(c)
        c = cups[c]
    return str(labels)


def next(cup: int, cups: Cups) -> Tuple[int, Cups]:
    removed = []
    ri = cups[cup]
    for i in range(3):
        removed.append(ri)
        ri = cups[ri]

    destination = cup
    while True:
        destination -= 1
        if destination <= 0:
            destination = N

        if not destination in removed:
            break

    debug("Destination cup: %d", destination)

    # Insert removed cups
    tmp = cups[destination]
    cups[destination] = removed[0]
    cups[removed[-1]] = tmp
    cups[cup] = ri

    # debug("After dest cup %d: %s", destination, print_cups(dest_cup))

    return cups[cup]


def play_rounds(cups: Cup, iterations: int) -> Cup:
    # memo = set()
    for i in range(iterations):
        #debug("Cups: %s", cups)
        if i % 100_000 == 0:
            print(".", end='')
        # state = tuple(cups)
        # if state in memo:
        #     raise Exception(f"Yo! i = {i}")
        # else:
        # memo.add(state)
        if isEnabled(logging.DEBUG):
            debug("Cups: %s", print_cups(cups, 10))
        cups = next(cups)
    return cups


def parse_cups(input: str) -> Cup:
    cup: Optional[Cup] = None
    returned_cup: Optional[Cup] = None
    for c in input:
        new = Cup(int(c))
        if cup is not None:
            cup.next = new
        else:
            returned_cup = new
        cup = new

    global N
    N = len(input)
    # Circle back to the 1st one
    if returned_cup is None:
        raise Exception("What !?")
    cup.next = returned_cup
    return returned_cup


def throw(error: str) -> Cup:
    raise Exception(error)


def find_cup(from_cup: Cup, label: int) -> Cup:
    if from_cup.label == label:
        return from_cup
    c: Cup = from_cup.next or throw("Null")
    if c.label == label:
        return c
    while c != from_cup:
        if c.label == label:
            return c
        else:
            c = c.next or throw("Null")
    raise Exception(f"Could not find label %d", label)


def part1(input: List[str]) -> str:
    cup = parse_cups(input[0])
    cup = play_rounds(cup, 100)

    one_cup = find_cup(cup, 1)
    labels = []
    c = one_cup.next
    while c != one_cup:
        labels.append(str(c.label))
        c = c.next
    return "".join(labels)


def part2(input: List[str]) -> int:
    cup = parse_cups(input[0])
    last = cup.next
    while last.next != cup:
        last = last.next

    global N
    for i in range(N, 1_000_000 + 1):
        last.next = Cup(i, cup)
        last = last.next
        N = i

    cups = play_rounds(cup, 10_000_000)

    one_cup = find_cup(cup, 1)
    return one_cup.next.label * one_cup.next.next.label
