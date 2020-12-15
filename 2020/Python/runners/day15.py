from typing import List
import itertools
from logging import debug


def get_nth(start, nth):
    numbers_turns = {}
    for n in start:
        numbers_turns[int(n)] = []
    last = start[-1]

    def play():
        i = 0
        for _ in range(len(start)):
            n = start[i]
            numbers_turns[n] = [i]
            yield n
            i += 1

        def produce(value: int) -> int:
            turns = numbers_turns.get(value)
            if turns is None:
                numbers_turns[value] = [i]
            else:
                numbers_turns[value] = [turns[-1], i]

            nonlocal last
            debug("Last value: %d, producing value %d; turns: %s",
                  last, value, numbers_turns)
            last = value
            return value

        while(True):
            turns = numbers_turns[last]
            if len(turns) == 1:
                yield produce(0)
            else:
                [t1, t2] = turns
                yield produce(t2-t1)
            i += 1
    return next(itertools.islice(play(), nth - 1, None))


def part1(input: List[str]) -> int:
    start = list(map(int, input[0].split(',')))
    return get_nth(start, 2020)


def part2(input: List[str]) -> int:
    start = list(map(int, input[0].split(',')))
    return get_nth(start, 30000000)
