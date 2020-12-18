import logging

from runners.utils import count, isEnabled
from typing import List, Set
from enum import Enum
from logging import debug, info


class Status(Enum):
    INACTIVE = 0
    ACTIVE = 1


Grid = List[List[List[List[Status]]]]


def read_grid(input: List[str]) -> Grid:
    def read_row(line: str):
        for ch in line:
            if ch == '.':
                yield Status.INACTIVE
            else:
                yield Status.ACTIVE
    return [[[list(read_row(line)) for line in input]]]


# def compare_grids(g1: Grid, g2: Grid) -> bool:
#     X = len(g1[0])
#     Y = len(g1)

#     for x in range(X):
#         for y in range(Y):
#             if g1[y][x] != g2[y][x]:
#                 return False
#     return True


def display_grid(g: Grid):
    X = len(g[0][0][0])
    Y = len(g[0][0])
    Z = len(g[0])
    W = len(g)

    def print_seat(seat: Status) -> str:
        if seat == Status.INACTIVE:
            return '.'
        if seat == Status.ACTIVE:
            return '#'
        raise Exception("Unknown")

    s = ""
    for w in range(W):
        for z in range(Z):
            s += f"\nz={z}, w={w}"
            for y in range(Y):
                s += "\n"
                s += "".join(print_seat(s) for s in g[w][z][y])

    return s


def has_active_border_cubes(g: Grid) -> bool:
    X = len(g[0][0][0])
    Y = len(g[0][0])
    Z = len(g[0])
    W = len(g)

    def has_active_cubes_on_plane(plane: List[List[Status]]) -> bool:
        return any(row for row in plane if has_active_cubes_on_row(row))

    def has_active_cubes_on_cube(cube: List[List[List[Status]]]) -> bool:
        return any(plane for plane in cube if has_active_cubes_on_plane(plane))

    def has_active_cubes_on_row(row: List[Status]) -> bool:
        return any(c for c in row if c == Status.ACTIVE)

    # Check w=0 and w = max
    if has_active_cubes_on_cube(g[0]) or has_active_cubes_on_cube(g[-1]):
        return True

    # Otherwise we want to know if any cube is on the border of any cube
    for w in range(1, W - 1):
        cube = g[w]
        if has_active_cubes_on_plane(cube[0]) or has_active_cubes_on_plane(cube[-1]):
            return True

        # Otherwise we want to know if any cube is on the border of any plane
        for z in range(1, Z - 1):
            plane = cube[z]
            # Check y = 0, y = max
            if has_active_cubes_on_row(plane[0]) or has_active_cubes_on_row(plane[-1]):
                return True

            for y in range(1, Y-1):
                if plane[y][0] == Status.ACTIVE or plane[y][-1] == Status.ACTIVE:
                    return True

    return False


def clone_grid(g: Grid) -> Grid:
    clone = [[[list(row) for row in plane] for plane in cube] for cube in g]
    if has_active_border_cubes(clone):
        X = len(g[0][0][0])
        Y = len(g[0][0])
        Z = len(g[0])
        W = len(g)

        # Add an extra layer in all direction:
        empty_cube1 = [
            [
                [Status.INACTIVE for x in range(X+2)]
                for y in range(Y+2)]
            for z in range(Z+2)]
        empty_cube2 = [
            [
                [Status.INACTIVE for x in range(X+2)]
                for y in range(Y+2)]
            for z in range(Z+2)]

        new_clone = [empty_cube1]
        for cube in clone:
            empty_plane1 = [
                [Status.INACTIVE for x in range(X+2)] for y in range(Y+2)]
            empty_plane2 = [
                [Status.INACTIVE for x in range(X+2)] for y in range(Y+2)]
            new_cube = [empty_plane1]
            for plane in cube:
                empty_row1 = [Status.INACTIVE for x in range(X+2)]
                empty_row2 = [Status.INACTIVE for x in range(X+2)]
                new_plane = [empty_row1]
                for row in plane:
                    new_row = [Status.INACTIVE]
                    for s in row:
                        new_row.append(s)
                    new_row.append(Status.INACTIVE)
                    new_plane.append(new_row)
                new_plane.append(empty_row2)
                new_cube.append(new_plane)
            new_cube.append(empty_plane2)
            new_clone.append(new_cube)
        new_clone.append(empty_cube2)
        clone = new_clone
    return clone


xds = [-1, 0, 1]
yds = [-1, 0, 1]
zds = [-1, 0, 1]
wds = [-1, 0, 1]


def next(g: Grid) -> Grid:
    def count_active_neighbours(x: int, y: int, z: int, w: int) -> int:
        return count(s for xd in xds
                     for yd in yds
                     for zd in zds
                     for wd in wds
                     for nx, ny, nz, nw in [(x+xd, y+yd, z+zd, w+wd)]
                     if nx >= 0 and ny >= 0 and nz >= 0 and nw >= 0 and nx < X and ny < Y and nz < Z and nw < W and (xd, yd, zd, wd) != (0, 0, 0, 0)
                     for s in [g[nw][nz][ny][nx]] if s == Status.ACTIVE)

    new = clone_grid(g)
    if has_active_border_cubes(new):
        raise Exception("What !?")

    if has_active_border_cubes(g):
        g = clone_grid(g)
    X = len(new[0][0][0])
    Y = len(new[0][0])
    Z = len(new[0])
    W = len(new)
    if isEnabled(logging.INFO):
        info("Current cloned grid: %s", display_grid(new))
    for x in range(X):
        for y in range(Y):
            for z in range(Z):
                for w in range(W):
                    c = count_active_neighbours(x, y, z, w)
                    seat = g[w][z][y][x]
                    #debug("Adjacent seats for %s (%s): %d", (x, y, z), seat, c)
                    if seat == Status.ACTIVE and not (c == 2 or c == 3):
                        new[w][z][y][x] = Status.INACTIVE
                    if seat == Status.INACTIVE and c == 3:
                        new[w][z][y][x] = Status.ACTIVE

    return new


def part1(input: List[str]) -> int:
    current = read_grid(input)
    for i in range(6):
        # priint(".", end='')
        if isEnabled(logging.INFO):
            info("Current grid: %s", display_grid(current))
        new = next(current)
        if isEnabled(logging.DEBUG):
            debug("New grid: %s", display_grid(new))

        current = new

    return sum(sum(sum(sum(1 for c in row if c == Status.ACTIVE) for row in plane) for plane in cube) for cube in current)


# def part2(input: List[str]) -> int:
#     current = read_grid(input)
#     while True:
#         # print(".", end='')
#         if isEnabled(logging.INFO):
#             info("Current grid: %s", display_grid(current))
#         new = next2(current)
#         if isEnabled(logging.DEBUG):
#             debug("New grid: %s", display_grid(new))
#         if compare_grids(current, new):
#             return sum(count(x for x in row if x == Status.OCCUPIED) for row in current)

#         current = new
