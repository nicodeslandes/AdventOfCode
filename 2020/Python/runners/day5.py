from runners.utils import findfirst, zipwithnext
from typing import List, Tuple
from logging import debug, info


def read_seat(line):
    row = 0
    for i in range(7):
        row = row * 2 + int(line[i] == 'B')

    col = 0
    for i in range(7, 10):
        col = col * 2 + int(line[i] == 'R')

    return row, col


def to_seat_id(seat: Tuple[int, int]):
    row, col = seat
    return row * 8 + col


def part1(input: List[str]) -> int:
    return max(row * 8 + col for line in input for row, col in [read_seat(line)])


def part2(input: List[str]) -> int:
    seats = [to_seat_id(read_seat(line)) for line in input]
    seats.sort()
    return findfirst(zipwithnext(seats), lambda t: t[1] != t[0] + 1)[0] + 1
