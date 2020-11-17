from logging import info
from typing import List


def part1(input: List[str]) -> int:
    input_range = input[0].split("-")
    [start, end] = [int(x) for x in input_range]
    info("Range: %d to %d", start, end)

    def check_number(num: int):
        adj_found = False
    
        next = num % 10
        num //= 10
        while num > 0:
            d = num % 10
            num //= 10

            if next < d:
                return False
        
            if next == d:
                adj_found = True

            next = d

        return adj_found


    count = 0
    for num in range(start, end + 1):
        if check_number(num): count += 1

    return count

def part2(input: List[str]) -> int:
    input_range = input[0].split("-")
    [start, end] = [int(x) for x in input_range]
    info("Range: %d to %d", start, end)

    def check_number(num: int):
        adj_found = False
        digits = list()

        while num > 0:
            d = num % 10
            num //= 10
            digits.insert(0, d)

        N = len(digits)
        for i in range(N-1):
            if digits[i] > digits[i+1]:
                return False
        
            if digits[i] == digits[i+1] and (i == 0 or digits[i-1] != digits[i]) and (i == N - 2 or digits[i+1] != digits[i+2]):
                adj_found = True

        return adj_found


    count = 0
    for num in range(start, end + 1):
        if check_number(num): count += 1

    return count
