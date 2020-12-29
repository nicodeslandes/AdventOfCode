from typing import List


def find_loop_size(subject: int, pub_key: int) -> int:
    value = 1
    for ls in range(1, 100_000_000):
        value = (value * subject) % 20201227
        if value == pub_key:
            return ls

    else:
        raise Exception("Failed to find loop size")


def transform(subject: int, loop_size: int) -> int:
    value = 1
    for _ in range(loop_size):
        value = (value * subject) % 20201227

    return value


def part1(input: List[str]) -> int:
    pkey1 = int(input[0])
    pkey2 = int(input[1])

    # find loop_size1
    ls1 = find_loop_size(7, pkey1)
    return transform(pkey2, ls1)

# def part2(input: List[str]) -> int:
