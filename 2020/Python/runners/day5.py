from typing import List
from logging import debug, info


def read_seat(line):
    row = 0
    for i in range(7):
        row = row * 2 + int(line[i] == 'B')

    col = 0
    for i in range(7, 10):
        col = col * 2 + int(line[i] == 'R')

    return row, col


def part1(input: List[str]) -> int:
    return max(row * 8 + col for line in input for row, col in [read_seat(line)])


def part2(input: List[str]) -> int:
    seats = [read_seat(line) for line in input]
    seats.sort()

    def find_missing_seat():
        for i in range(1, len(seats)):
            (r1, c1) = seats[i]
            (r2, c2) = seats[i+1]
            if r1 == r2:
                if c2 != c1 + 1:
                    return r1, c1 + 1
            else:
                if c1 != 7:
                    return r1, 7
                if c2 != 0:
                    return r2, 0

        raise Exception("Couldn't find missing seat")

    r, c = find_missing_seat()
    info("Found seat: %s", (r, c))
    return r * 8 + c
