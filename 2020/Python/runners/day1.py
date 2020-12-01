from typing import List

def part1(input: List[str]) -> int:
    values = [int(x) for x in input]
    for i in range(len(values)):
        for j in range(i, len(values)):
            if values[i] + values[j] == 2020:
                return values[i] * values[j]
    return 0

# def part2(input: List[str]) -> int:
