from typing import List
from runners.utils import count


def part1(input: List[str]) -> int:
    count = 0

    def process_group(group: List[str]):
        answers = {}
        for pax in group:
            for answer in pax:
                answers[answer] = 1

        nonlocal count
        count += len(answers)

    group = []
    for line in input:
        if len(line) == 0:
            process_group(group)
            group.clear()
        group.append(line)

    process_group(group)
    return count


def part2(input: List[str]) -> int:
    total = 0

    def process_group(group: List[str]):
        answers = {}
        for pax in group:
            for answer in pax:
                c = answers.get(answer)
                answers[answer] = c + 1 if c else 1

        gl = len(group)
        nonlocal total
        s = count(1 for answer in answers if answers[answer] == gl)
        total += s

    group = []
    for line in input:
        if len(line) == 0:
            process_group(group)
            group.clear()
        else:
            group.append(line)

    process_group(group)
    return total
