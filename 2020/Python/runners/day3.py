from logging import debug
from typing import List


def part1(input: List[str]) -> int:
    def read_row(line: str):
        return [ch == '#' for ch in line]

    grid = [read_row(line.strip()) for line in input]

    pos = (0, 0)
    delta = (3, 1)
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

# def part2(input: List[str]) -> int:
