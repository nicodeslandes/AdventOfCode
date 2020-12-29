import logging
from typing import Callable, Dict, Iterable, Iterator, List, Optional, Tuple
from logging import debug, info
from enum import Enum
from runners.utils import isEnabled, product

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
            tile.append([1 if ch == '#' else 0 for ch in line])
    return tiles


def display_tile(tile: Tile):
    if isEnabled(logging.DEBUG):
        for row in tile:
            debug("%s", "".join(('▒▒' if c == 0 else '██' for c in row)))


class Direction(Enum):
    U = 1
    D = 2
    L = 3
    R = 4


Pos = Tuple[int, int]
side_accessors: Dict[Direction, Callable[[Tile], List[int]]] = {
    Direction.U: lambda t: t[0],
    Direction.D: lambda t: t[-1],
    Direction.L: lambda t: [c for y in range(len(t[0])) for c in [t[y][0]]],
    Direction.R: lambda t: [c for y in range(len(t[0])) for c in [t[y][-1]]]
}


def get_side(tile: Tile, side: Direction) -> List[int]:
    return side_accessors[side](tile)


def rotate(tile: Tile) -> Tile:
    X = len(tile[0])
    Y = len(tile)
    return [
        [tile[y][Y-x-1] for y in range(X)]
        for x in range(Y)
    ]


def flip_horiz(tile: Tile) -> Tile:
    return [r[::-1] for r in tile]


def flip_vert(tile: Tile) -> Tile:
    return tile[::-1]


def get_transformations(tile: Tile):
    def get_rotate_transforms():
        yield tile
        t = tile
        for i in range(3):
            t = rotate(t)
            yield t

    for t in get_rotate_transforms():
        yield t
        yield flip_horiz(t)
        yield flip_vert(t)


def match(tile1: Tile, tile2: Tile) -> Optional[Tuple[Pos, Tile]]:
    X = len(tile1[0])
    Y = len(tile1)

    def get_sides() -> Iterator[Tuple[Pos, Callable[[Tile, Tile], bool]]]:
        yield (0, 1), lambda t1, t2: get_side(t1, Direction.U) == get_side(t2, Direction.D)
        yield (0, -1), lambda t1, t2: get_side(t1, Direction.D) == get_side(t2, Direction.U)
        yield (-1, 0), lambda t1, t2: get_side(t1, Direction.L) == get_side(t2, Direction.R)
        yield (1, 0), lambda t1, t2: get_side(t1, Direction.R) == get_side(t2, Direction.L)

    # Try to match up all transformations of tile2
    for t2 in get_transformations(tile2):
        # In all directions
        for pos, comparison in get_sides():
            if comparison(tile1, t2):
                # Found match
                # debug("Matching tiles; pos: %s", pos)
                # display_tile(tile1)
                # debug("and")
                # display_tile(t2)
                comparison(tile1, t2)
                return pos, t2
    return None


def arrange_tiles(tiles):
    tile_grid: Dict[Pos, Tile] = {}
    tile_id_grid: Dict[Pos, int] = {}
    unmatched_tiles = list(tiles.keys())

    # Start with the first tile, and place it at 0,0 in the tile grid
    tile_id_grid[(0, 0)] = unmatched_tiles.pop()
    tile_grid[(0, 0)] = tiles[tile_id_grid[(0, 0)]]

    while any(unmatched_tiles):
        # Find a unique tile that lines up with any other from the grid
        for grid_tile_id, grid_tile in list(tile_grid.items()):
            for tile_id in unmatched_tiles:
                tile = tiles[tile_id]
                if m := match(grid_tile, tile):
                    (pos, matched_tile) = m
                    grid_pos = (grid_tile_id[0] + pos[0],
                                grid_tile_id[1] + pos[1])
                    tile_grid[grid_pos] = matched_tile
                    tile_id_grid[grid_pos] = tile_id
                    unmatched_tiles.remove(tile_id)
                    break

    return tile_grid, tile_id_grid


def find_grid_min_max(coordinates):
    min_x, max_x, min_y, max_y = 0, 0, 0, 0
    for (x, y) in coordinates:
        min_x = min(x, min_x)
        max_x = max(x, max_x)
        min_y = min(y, min_y)
        max_y = max(y, max_y)

    return min_x, max_x, min_y, max_y


def part1(input: List[str]) -> int:
    tiles = parse_tiles(input)
    tile_grid, tile_id_grid = arrange_tiles(tiles)
    min_x, max_x, min_y, max_y = find_grid_min_max(tile_id_grid.keys())

    return product(tile_id_grid[(x, y)] for x in (min_x, max_x) for y in (min_y, max_y))


def parse_sea_monster() -> Tile:
    return parse_tiles(
        [
            "Tile 0:",
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   "]
    )[0]


def part2(input: List[str]) -> int:
    tiles = parse_tiles(input)
    tile_grid, tile_id_grid = arrange_tiles(tiles)
    min_x, max_x, min_y, max_y = find_grid_min_max(tile_id_grid.keys())

    tile_x = len(tile_grid[(0, 0)][0])
    tile_y = len(tile_grid[(0, 0)])
    picture = [[0 for x in range((max_x - min_x + 1) * (tile_x - 2))]
               for y in range((max_y - min_y + 1) * (tile_y - 2))]
    for y in range(max_y, min_y - 1, -1):
        for x in range(min_x, max_x + 1):
            tile = tile_grid[(x, y)]
            for cx in range(1, tile_x - 1):
                for cy in range(1, tile_y - 1):
                    py = (y - min_y + 1) * (tile_y-2) - cy
                    px = (x-min_x) * (tile_x-2) + cx - 1
                    picture[py][px] = tile[cy][cx]

    X = (tile_x - 2) * (max_x - min_x + 1)
    Y = (tile_y - 2) * (max_y - min_y + 1)
    for y in range(Y-1, -1, -1):
        row = ['██' if picture[y][x] else '▒▒' for x in range(X)]
        debug("".join(row))

    sea_monster = parse_sea_monster()
    SX = len(sea_monster[0])
    SY = len(sea_monster)

    def is_sea_monster_at(t: Tile, pos: Pos):
        for r in range(SY):
            for sx in range(SX):
                sy = -r
                if sea_monster[r][sx] == 1 and t[pos[1] + sy][pos[0] + sx] != 1:
                    return False
        return True

    count = 0
    for t in get_transformations(picture):
        debug("Checking for sea monsters in:")
        display_tile(t)
        if t[0][:5] == [0, 1, 1, 1, 1]:
            breakpoint()

        for y in range(Y-1, SY-1, -1):
            for x in range(0, X-SX):
                if is_sea_monster_at(t, (x, y)):
                    count += 1
        debug("Found %d monsters", count)
        if count != 0:
            break

    info("Sea monster count: %d", count)
    sea_monster_cells = count * \
        sum(sea_monster[y][x] for x in range(SX) for y in range(SY))
    picture_cells = sum(picture[y][x] for x in range(X) for y in range(Y))

    return picture_cells - sea_monster_cells
