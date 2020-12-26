from typing import List
from logging import debug, info

movements = [
    ('se', (1, -1)),
    ('sw', (-1, -1)),
    ('ne', (1, 1)),
    ('nw', (-1, 1)),
    ('e', (2, 0)),
    ('w', (-2, 0)),
]


def get_tile(line: str):
    i = 0
    pos = (0, 0)
    while i < len(line):
        for mstr, (x, y) in movements:
            if line.startswith(mstr, i):
                pos = (pos[0] + x, pos[1] + y)
                debug("Move: %s, pos: %s", mstr, pos)
                i += len(mstr)
                break
        else:
            raise Exception(
                "Failed to parse string: no match for " + line[i:i+2])
    info("New tile: %s", pos)
    return pos


def part1(input: List[str]) -> int:
    tile_flips = {}
    for line in input:
        tile = get_tile(line)
        flips = tile_flips.get(tile)
        if flips is None:
            tile_flips[tile] = 1
        else:
            tile_flips[tile] = flips + 1

    debug("Tiles: %s", tile_flips)
    return sum(x for x in tile_flips.values() if x % 2 == 1)

# def part2(input: List[str]) -> int:
