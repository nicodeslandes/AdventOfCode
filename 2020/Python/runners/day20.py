from typing import Dict, List
from logging import debug

Tile = List[List[int]]


def parse_tiles(input: List[str]) -> Dict[int, Tile]:
    tile_id = 0
    tile = []
    tiles = {}
    for line in input:
        if line.startswith("Tile"):
            tile_id = int(line.split(' ')[1][:-1])
            tile = []
            tiles[tile_id] = tile
        elif line != "":
            tile.append([0 if ch == '.' else 1 for ch in line])
    return tiles


def display_tile(tile: Tile):
    for row in tile:
        debug("%s", "".join(('.' if c == 0 else '#' for c in row)))


def match(tile1: Tile, tile2: Tile):
    X = len(tile1[0])
    Y = len(tile1)

    # xds = [-1, 1]
    # yds = [-1, 1]
    # for xd in xds:
    #     for yd in yds:
    #         # Attempt a match with tile2 sitting at pos (xd,yd) relative to tile1

    def get_sides(tile: Tile):
        yield (0, 1), tile[0]
        yield (0, -1), tile[-1]
        yield (-1, 0), [c for y in range(Y) for c in [tile[y][0]]]
        yield (1, 0), [c for y in range(Y) for c in [tile[y][-1]]]

    # Try to match up sides
    for id1, side1 in get_sides(tile1):
        for id2, side2 in get_sides(tile2):
            if side1 == side2 or side1 == list(reversed(side2)):
                is_reversed = side1 != side2
                if id1 == 'U' and id2 == 'D':
                    return True, (0, 1), (int(is_reversed), 0)
                if id1 == 'U' and id2 == 'U':

                if (id1, id2) = ''
                #

    return False


def part1(input: List[str]) -> int:
    tiles = parse_tiles(input)
    tile_grid = {}
    unmatched_tiles = list(tiles.keys())

    # Start with the first tile, and place it at 0,0 in the tile grid
    tile_grid[(0, 0)] = unmatched_tiles.pop()

    while any(unmatched_tiles):
        # Find a unique tile that lines up with any other from the grid
        for grid_tile_id, grid_tile in tile_grid.items():
            for tile_id in unmatched_tiles:
                tile = tiles[tile_id]
                if m := match(grid_tile, tile):

    return 0

# def part2(input: List[str]) -> int:
