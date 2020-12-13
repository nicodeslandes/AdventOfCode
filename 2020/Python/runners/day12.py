from typing import List
from logging import debug
import numpy as np


class Action:
    def __init__(self, d: str, dist: int):
        self.dir = d
        self.dist = dist


cardinal_dirs_R = {
    'N': 'E',
    'E': 'S',
    'S': 'W',
    'W': 'N',
}

cardinal_dirs_L = {
    'N': 'W',
    'E': 'N',
    'S': 'E',
    'W': 'S',
}

deltas = {
    'N': (0, -1),
    'E': (1, 0),
    'S': (0, 1),
    'W': (-1, 0),
}


class Ship:
    def __init__(self):
        self.direction = 'E'
        self.pos = np.zeros(2)
        self.waypoint = np.array((10, -1))

    def move(self, action: Action):
        debug("Pos: %s, Movement: %s,%d", self.pos, action.dir, action.dist)
        if action.dir in "NESW":
            delta = deltas[action.dir]
            self.pos += np.array(delta) * action.dist
        elif action.dir == 'F':
            self.move(Action(self.direction, action.dist))
        elif action.dir == 'R':
            for _ in range(action.dist // 90):
                self.direction = cardinal_dirs_R[self.direction]
        elif action.dir == 'L':
            for _ in range(action.dist // 90):
                self.direction = cardinal_dirs_L[self.direction]
        else:
            raise Exception(f'Huh? {action.dir}')

    def move2(self, action: Action):
        debug("Pos: %s, WP: %s, Movement: %s,%d", self.pos,
              self.waypoint, action.dir, action.dist)
        if action.dir in "NESW":
            delta = deltas[action.dir]
            self.waypoint += np.array(delta) * action.dist
        elif action.dir == 'F':
            self.pos += self.waypoint * action.dist
        elif action.dir == 'R':
            for _ in range(action.dist // 90):
                x, y = self.waypoint
                self.waypoint = np.array((-y, x))
        elif action.dir == 'L':
            for _ in range(action.dist // 90):
                x, y = self.waypoint
                self.waypoint = np.array((y, -x))
        else:
            raise Exception(f'Huh? {action.dir}')


def part1(input: List[str]) -> int:
    s = Ship()
    for line in input:
        action = Action(line[0], int(line[1:]))
        s.move(action)
    return int(abs(s.pos[0]) + abs(s.pos[1]))


def part2(input: List[str]) -> int:
    s = Ship()
    for line in input:
        action = Action(line[0], int(line[1:]))
        s.move2(action)
    return int(abs(s.pos[0]) + abs(s.pos[1]))
