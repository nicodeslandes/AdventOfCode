from typing import List
from logging import debug


def is_pair_sum(n: int, last_n: List[int]) -> bool:
    debug("Checking %d", n)
    for i in range(len(last_n)):
        for j in range(i + 1, len(last_n)):
            if last_n[i] + last_n[j] == n:
                debug("%d is sum of %d and %d", n, last_n[i], last_n[j])
                return True
    return False


def part1(input: List[str], is_test: bool) -> int:
    preamble_length = 5 if is_test else 25
    numbers = (int(x) for x in input)
    last_n = []
    for n in numbers:
        if len(last_n) < preamble_length:
            last_n.append(n)
            continue

        if not is_pair_sum(n, last_n):
            return n
        last_n.append(n)
        last_n.pop(0)

    return -1

# def part2(input: List[str]) -> int:
