from logging import debug
from typing import List


def read_grid(input):
    def read_row(line: str):
        return [ch == '#' for ch in line]

    return [read_row(line.strip()) for line in input]


def count_trees(grid, delta):
    x = 0
    y = 0
    count = 0
    xmax = len(grid[0])
    ymax = len(grid)
    while y < ymax:
        debug("Checking %s: %s", (x, y), grid[y][x])
        if grid[y][x]:
            count += 1
        y += delta[1]
        x = (x + delta[0]) % xmax

    return count


def part1(input: List[str]) -> int:
    grid = read_grid(input)
    delta = (3, 1)
    return count_trees(grid, delta)


def part2(input: List[str]) -> int:
    grid = read_grid(input)
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    result = 1
    for d in slopes:
        result *= count_trees(grid, d)
    return result
