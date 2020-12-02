from typing import List


class PasswordPolicy:
    def __init__(self, min: int, max: int, char: str):
        self.min = min
        self.max = max
        self.char = char

    def check_password(self, password: str):
        occurrences = password.count(self.char)
        return occurrences >= self.min and occurrences <= self.max


class PasswordPolicy2:
    def __init__(self, index1: int, index2: int, char: str):
        self.index1 = index1
        self.index2 = index2
        self.char = char

    def check_password(self, password: str):
        check1 = password[self.index1 - 1] == self.char
        check2 = password[self.index2 - 1] == self.char
        return check1 ^ check2


def part1(input: List[str]) -> int:
    parsed = []
    for line in input:
        tokens = line.strip().split(' ')
        min_max = tokens[0].split('-')
        policy = PasswordPolicy(int(min_max[0]), int(min_max[1]), tokens[1][0])
        parsed.append((policy, tokens[2]))
    return sum([int(policy.check_password(x)) for policy, x in parsed])


def part2(input: List[str]) -> int:
    parsed = []
    for line in input:
        tokens = line.strip().split(' ')
        min_max = tokens[0].split('-')
        policy = PasswordPolicy2(
            int(min_max[0]), int(min_max[1]), tokens[1][0])
        parsed.append((policy, tokens[2]))
    return sum([int(policy.check_password(x)) for policy, x in parsed])
