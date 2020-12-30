import logging
from typing import List
from runners.utils import isEnabled
from logging import debug


def get_nth(start, nth):
    numbers_turns: List[int] = [0xFFFFFFFFFFFFFFFF] * nth
    last = start[-1]

    i = 0
    for _ in range(len(start)):
        n = start[i]
        numbers_turns[n] = 0xFFFFFFFF00000000 | i
        i += 1

    def produce(value: int) -> int:
        turns = numbers_turns[value]
        numbers_turns[value] = ((turns << 32) | i) & 0xFFFFFFFFFFFFFFFF
        return value

    while(i < nth):
        turns = numbers_turns[last]
        first = turns >> 32
        if first == 0xFFFFFFFF:
            last = produce(0)
        else:
            last = produce((turns & 0xFFFFFFFF) - first)
        i += 1

    return last


def part1(input: List[str]) -> int:
    start = list(map(int, input[0].split(',')))
    return get_nth(start, 2020)


def part2(input: List[str]) -> int:
    start = list(map(int, input[0].split(',')))
    return get_nth(start, 30000000)
