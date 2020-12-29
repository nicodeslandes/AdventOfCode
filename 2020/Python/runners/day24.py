from typing import Dict, Iterable, List, Tuple
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
                #debug("Move: %s, pos: %s", mstr, pos)
                i += len(mstr)
                break
        else:
            raise Exception(
                "Failed to parse string: no match for " + line[i:i+2])
    info("New tile: %s", pos)
    return pos


def parse_tiles(input):
    tile_flips = {}
    for line in input:
        tile = get_tile(line)
        flips = tile_flips.get(tile)
        if flips is None:
            tile_flips[tile] = 1
        else:
            tile_flips[tile] = 1 - flips
    return tile_flips


def part1(input: List[str]) -> int:
    tiles = parse_tiles(input)
    debug("Tiles: %s", tiles)
    return sum(x for x in tiles.values() if x == 1)


Pos = Tuple[int, int]
Tiles = Dict[Pos, int]


def adjacent_positions(pos: Pos) -> Iterable[Pos]:
    for _, (x, y) in movements:
        yield (pos[0] + x, pos[1] + y)


def next(tiles: Tiles) -> Tiles:
    existing_tiles = tiles
    tiles = tiles.copy()
    p = existing_tiles.keys().__iter__().__next__()
    (min_x, min_y) = p
    (max_x, max_y) = p
    for (x, y) in existing_tiles.keys():
        min_x = min(x, min_x)
        min_y = min(y, min_y)
        max_x = max(x, max_x)
        max_y = max(y, max_y)

    for x in range(min_x - 1, (max_x + 1 - min_x)):
        y = min_y - 1
        if x % 2 == 0 and y % 2 == 1 or x % 2 == 1 and y % 2 == 0:
            y -= 1
        while y <= max_y + 1:
            tile = (x, y)
            colour = existing_tiles.get(tile)
            black_tiles_count = sum(1
                                    for pos in adjacent_positions(tile)
                                    for cell in [existing_tiles.get(pos)]
                                    if cell == 1)
            if colour == 1:
                if black_tiles_count == 0 or black_tiles_count > 2:
                    del tiles[tile]
            else:
                if black_tiles_count == 2:
                    tiles[tile] = 1
            y += 2
    return tiles


def part2(input: List[str]) -> int:
    tiles = parse_tiles(input)
    for i in range(100):
        tiles = next(tiles)
        debug("Day %d: %d", i, sum(1 for x in tiles.values() if x == 1))
    return sum(1 for x in tiles.values() if x == 1)
