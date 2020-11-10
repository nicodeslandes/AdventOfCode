from logging import debug, info
from typing import List, Optional, Set, Tuple

Position = Tuple[int, int]
Grid = dict[Position, int]

def parse_grid(s: str) -> Grid:
    grid: Grid = {}
    (x,y) = (0,0)
    steps = 0
    for move in s.split(','):
        debug("Move: %s", move)
        delta = int(move[1:])

        for _ in range(delta):
            if move[0] == 'R':
                x += 1
            elif move[0] == 'L':
                x -= 1
            elif move[0] == 'U':
                y += 1
            elif move[0] == 'D':
                y -= 1
            else:
                raise Exception("Unknown move")

            steps += 1
            if not (x,y) in grid:
                grid[(x,y)] = steps

    debug("Grid: %s", grid)
    return grid

def part1(input: List[str]) -> int:
    grid1 = parse_grid(input[0])
    grid2 = parse_grid(input[1])

    intersections = set(grid1.keys()).intersection(grid2.keys())
    info("Found intersections: %s", intersections)
    m = min([abs(x)+abs(y) for (x,y) in intersections])
    return m


def part2(input: List[str]) -> int:
    grid1 = parse_grid(input[0])
    grid2 = parse_grid(input[1])

    intersections = set(grid1.keys()).intersection(grid2.keys())
    info("Found intersections: %s", intersections)
    m = min([grid1[p] + grid2[p] for p in intersections])
    return m
