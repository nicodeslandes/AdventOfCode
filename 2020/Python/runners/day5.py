from typing import List


def read_seat(line):
    row = 0
    for i in range(7):
        row = row * 2 + int(line[i] == 'B')

    col = 0
    for i in range(7, 10):
        col = col * 2 + int(line[i] == 'R')

    return row * 8 + col


def part1(input: List[str]) -> int:
    return max(read_seat(line) for line in input)


def part2(input: List[str]) -> int:
