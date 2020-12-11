import logging
from runners.utils import count, isEnabled
from typing import List
from enum import Enum
from logging import debug, info


class Status(Enum):
    FREE = 0
    FLOOR = 1
    OCCUPIED = 2


Grid = List[List[Status]]


def read_grid(input: List[str]) -> Grid:
    def read_row(line: str):
        for ch in line:
            if ch == 'L':
                yield Status.FREE
            else:
                yield Status.FLOOR
    return [list(read_row(line)) for line in input]


def compare_grids(g1: Grid, g2: Grid) -> bool:
    X = len(g1[0])
    Y = len(g1)

    for x in range(X):
        for y in range(Y):
            if g1[y][x] != g2[y][x]:
                return False
    return True


def display_grid(g: Grid):
    X = len(g[0])
    Y = len(g)

    def print_seat(seat: Status) -> str:
        if seat == Status.FLOOR:
            return '.'
        if seat == Status.FREE:
            return 'L'
        if seat == Status.OCCUPIED:
            return '#'
        raise Exception("Unknown")

    s = ""
    for y in range(Y):
        s += "\n"
        s += "".join(print_seat(s) for s in g[y])

    return s


def clone_grid(g: Grid) -> Grid:
    return [list(row) for row in g]


def next(g: Grid) -> Grid:
    def count_adj_occupied_seats(x: int, y: int) -> int:
        xds = [-1, 0, 1]
        yds = [-1, 0, 1]
        return count(s for xd in xds
                     for yd in yds
                     for nx, ny in [(x+xd, y+yd)]
                     if nx >= 0 and nx < X and ny >= 0 and ny < Y and (xd, yd) != (0, 0)
                     for s in [g[ny][nx]] if s == Status.OCCUPIED)

    X = len(g[0])
    Y = len(g)
    new = clone_grid(g)
    for x in range(X):
        for y in range(Y):
            c = count_adj_occupied_seats(x, y)
            seat = g[y][x]
            debug("Adjacent seats for %s (%s): %d", (x, y), seat, c)
            if seat == Status.FREE and c == 0:
                new[y][x] = Status.OCCUPIED
            if seat == Status.OCCUPIED and c >= 4:
                new[y][x] = Status.FREE

    return new


xds = [-1, 0, 1]
yds = [-1, 0, 1]


def next2(g: Grid) -> Grid:
    def count_visible_occupied_seats(x: int, y: int) -> int:
        c = 0
        for (xd, yd) in ((xd, yd) for xd in xds for yd in yds if (xd, yd) != (0, 0)):
            nx = x + xd
            ny = y + yd
            while nx >= 0 and nx < X and ny >= 0 and ny < Y:
                if g[ny][nx] == Status.FLOOR:
                    nx += xd
                    ny += yd
                else:
                    if g[ny][nx] == Status.OCCUPIED:
                        c += 1
                    break
        return c

    X = len(g[0])
    Y = len(g)
    new = clone_grid(g)
    for x in range(X):
        for y in range(Y):
            c = count_visible_occupied_seats(x, y)
            seat = g[y][x]
            debug("Visible occupied seats for %s (%s): %d", (x, y), seat, c)
            if seat == Status.FREE and c == 0:
                new[y][x] = Status.OCCUPIED
            if seat == Status.OCCUPIED and c >= 5:
                new[y][x] = Status.FREE

    return new


def part1(input: List[str]) -> int:
    current = read_grid(input)
    while True:
        #print(".", end='')
        if isEnabled(logging.INFO):
            info("Current grid: %s", display_grid(current))
        new = next(current)
        if isEnabled(logging.DEBUG):
            debug("New grid: %s", display_grid(new))
        if compare_grids(current, new):
            return sum(count(x for x in row if x == Status.OCCUPIED) for row in current)

        current = new


def part2(input: List[str]) -> int:
    current = read_grid(input)
    while True:
        #print(".", end='')
        if isEnabled(logging.INFO):
            info("Current grid: %s", display_grid(current))
        new = next2(current)
        if isEnabled(logging.DEBUG):
            debug("New grid: %s", display_grid(new))
        if compare_grids(current, new):
            return sum(count(x for x in row if x == Status.OCCUPIED) for row in current)

        current = new
