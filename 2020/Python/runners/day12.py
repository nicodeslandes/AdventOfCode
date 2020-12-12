from typing import List
from enum import Enum
from logging import debug


class Direction(Enum):
    N = 0
    S = 1
    E = 2
    W = 3
    L = 4
    R = 5
    F = 6


class Action:
    def __init__(self, d: Direction, dist: int):
        self.dir = d
        self.dist = dist


cardinal_dirs_R = {
    Direction.N: Direction.E,
    Direction.E: Direction.S,
    Direction.S: Direction.W,
    Direction.W: Direction.N
}

cardinal_dirs_L = {
    Direction.N: Direction.W,
    Direction.E: Direction.N,
    Direction.S: Direction.E,
    Direction.W: Direction.S
}


class Ship:
    def __init__(self):
        self.direction = Direction.E
        self.pos = 0, 0
        self.waypoint = 10, -1

    def move(self, action: Action):
        debug("Pos: %s, Movement: %s,%d", self.pos, action.dir, action.dist)
        x, y = self.pos
        if action.dir == Direction.N:
            self.pos = x, y - action.dist
        elif action.dir == Direction.S:
            self.pos = x, y + action.dist
        elif action.dir == Direction.E:
            self.pos = x + action.dist, y
        elif action.dir == Direction.W:
            self.pos = x - action.dist, y
        elif action.dir == Direction.F:
            self.move(Action(self.direction, action.dist))
        elif action.dir == Direction.R:
            for _ in range(action.dist // 90):
                self.direction = cardinal_dirs_R[self.direction]
        elif action.dir == Direction.L:
            for _ in range(action.dist // 90):
                self.direction = cardinal_dirs_L[self.direction]
        else:
            raise Exception(f'Huh? {action.dir}')

    def move2(self, action: Action):
        debug("Pos: %s, WP: %s, Movement: %s,%d", self.pos,
              self.waypoint, action.dir, action.dist)
        x, y = self.waypoint
        if action.dir == Direction.N:
            self.waypoint = x, y - action.dist
        elif action.dir == Direction.S:
            self.waypoint = x, y + action.dist
        elif action.dir == Direction.E:
            self.waypoint = x + action.dist, y
        elif action.dir == Direction.W:
            self.waypoint = x - action.dist, y
        elif action.dir == Direction.F:
            px, py = self.pos
            px += x * action.dist
            py += y * action.dist
            self.pos = px, py
        elif action.dir == Direction.R:
            if action.dist % 90 != 0 or action.dist < 0:
                raise Exception(f"What??? {action.dist}")
            for _ in range(action.dist // 90):
                x, y = self.waypoint
                self.waypoint = -y, x
        elif action.dir == Direction.L:
            if action.dist % 90 != 0 or action.dist < 0:
                raise Exception(f"What??? {action.dist}")
            for _ in range(action.dist // 90):
                x, y = self.waypoint
                self.waypoint = y, -x
        else:
            raise Exception(f'Huh? {action.dir}')

        debug("Pos: %s, WP: %s, Movement: %s,%d", self.pos,
              self.waypoint, action.dir, action.dist)


def part1(input: List[str]) -> int:
    s = Ship()
    for line in input:
        action = Action(Direction[line[0]], int(line[1:]))
        s.move(action)
    return abs(s.pos[0]) + abs(s.pos[1])


def part2(input: List[str]) -> int:
    s = Ship()
    for line in input:
        action = Action(Direction[line[0]], int(line[1:]))
        s.move2(action)
    return abs(s.pos[0]) + abs(s.pos[1])
