from typing import List
import itertools
from logging import debug


def get_nth(start, nth):
    numbers_turns: List[List[int]] = [[-1, -1] for _ in range(nth)]
    last = start[-1]

    i = 0
    for _ in range(len(start)):
        n = start[i]
        numbers_turns[n][1] = i
        i += 1

    def produce(value: int) -> int:
        turns = numbers_turns[value]
        turns[0] = turns[1]
        turns[1] = i

        nonlocal last
        debug("Last value: %d, producing value %d; turns: %s",
              last, value, numbers_turns)
        last = value
        return value

    while(i < nth):
        turns = numbers_turns[last]
        first = turns[0]
        if first == -1:
            produce(0)
        else:
            produce(turns[1] - first)
        i += 1

    return last


def part1(input: List[str]) -> int:
    start = list(map(int, input[0].split(',')))
    return get_nth(start, 2020)


def part2(input: List[str]) -> int:
    start = list(map(int, input[0].split(',')))
    return get_nth(start, 30000000)
