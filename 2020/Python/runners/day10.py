from typing import List
from logging import debug, info


def part1(input: List[str]) -> int:
    jolts = sorted([int(x) for x in input])
    diffs = {}
    jolts.insert(0, 0)
    for i in range(len(jolts) - 1):
        d = jolts[i+1] - jolts[i]
        debug("Diff: %d", d)
        if not d in diffs:
            diffs[d] = 1
        else:
            diffs[d] += 1

    diffs[3] += 1
    info("Diffs: %s", diffs)
    return diffs[1] * diffs[3]

# def part2(input: List[str]) -> int:
