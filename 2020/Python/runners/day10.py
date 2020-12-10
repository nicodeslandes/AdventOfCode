from typing import Dict, Generator, List
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


def part2(input: List[str]) -> int:
    jolts = sorted([int(x) for x in input])
    diffs = {}
    jolts.insert(0, 0)
    jolts.append(max(jolts) + 3)

    debug("Jolts: %s", jolts)

    for i in range(len(jolts) - 1):
        d = jolts[i+1] - jolts[i]
        debug("Diff: %d", d)
        if not d in diffs:
            diffs[d] = 1
        else:
            diffs[d] += 1

    #diffs[3] += 1
    info("Diffs: %s", diffs)

    def get_jolt_diff(index: int):
        current = jolts[index]
        # if index >= len(jolts) - 1:
        #     return 3
        # else:
        next = jolts[index + 1]
        return next - current

    memo: Dict[int, int] = {}

    def combination_count(start_index: int) -> int:
        cached = memo.get(start_index)
        if cached is not None:
            return cached

        if start_index == len(jolts) - 1:
            return 1
        if start_index >= len(jolts):
            return 0

        diff = get_jolt_diff(start_index)

        def get_result():
            if diff == 3:
                return combination_count(start_index + 1)
            else:
                next_diff = get_jolt_diff(start_index + 1)
                if next_diff == 1:
                    next_next_diff = get_jolt_diff(start_index + 2)
                    if next_next_diff == 1:
                        # We have 1-1-1, we can skip current, and next
                        return combination_count(start_index + 1) + combination_count(start_index+2) + combination_count(start_index+3)
                    else:
                        # We have 1-1-3 diffs, can skip current
                        return combination_count(start_index + 1) + combination_count(start_index+2)
                else:
                    # We have 1-3
                    return combination_count(start_index + 1)

        r = get_result()
        debug("Count for index %d (%d): %d",
              start_index, jolts[start_index], r)
        memo[start_index] = r
        return r

    return combination_count(0)
