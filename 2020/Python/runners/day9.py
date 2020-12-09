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

    raise Exception("Could not find weakness")


def part2(input: List[str], is_test: bool) -> int:
    preamble_length = 5 if is_test else 25
    numbers = list(int(x) for x in input)
    last_n = []
    weakness = None
    for n in numbers:
        if len(last_n) < preamble_length:
            last_n.append(n)
            continue

        if not is_pair_sum(n, last_n):
            weakness = n
        last_n.append(n)
        last_n.pop(0)

    if not weakness:
        raise Exception("Could not find weakness")

    prefixes = []
    psum = 0
    for n in numbers:
        psum += n
        prefixes.append(psum)

    for i in range(len(prefixes)):
        for j in range(i):
            if prefixes[i] - prefixes[j] == weakness:
                debug("Found chain! From %d to %d", numbers[i], numbers[j+1])
                mi = min([numbers[k] for k in range(j+1, i)])
                ma = max([numbers[k] for k in range(j+1, i)])
                return mi + ma

    raise Exception("Failed")
