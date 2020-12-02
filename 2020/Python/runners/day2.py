from abc import ABC, abstractmethod
from typing import List, Tuple, Type, TypeVar


class PasswordPolicy(ABC):
    @abstractmethod
    def check_password(self, password: str) -> bool: pass


class PasswordPolicy1(PasswordPolicy):
    def __init__(self, min: int, max: int, char: str):
        self.min = min
        self.max = max
        self.char = char

    def check_password(self, password: str):
        occurrences = password.count(self.char)
        return occurrences >= self.min and occurrences <= self.max


class PasswordPolicy2(PasswordPolicy):
    def __init__(self, index1: int, index2: int, char: str):
        self.index1 = index1
        self.index2 = index2
        self.char = char

    def check_password(self, password: str):
        check1 = password[self.index1 - 1] == self.char
        check2 = password[self.index2 - 1] == self.char
        return check1 ^ check2


T = TypeVar("T", bound=PasswordPolicy)


def parse_line(policy_class: Type[T], line: str) -> Tuple[PasswordPolicy, str]:
    tokens = line.strip().split(' ')
    parameters = list(map(int, tokens[0].split('-')))
    policy = policy_class(
        parameters[0], parameters[1], tokens[1][0])  # type: ignore
    return policy, tokens[2]


def part1(input: List[str]) -> int:
    return len([0 for line in input
                for policy, password in [parse_line(PasswordPolicy1, line)]
                if policy.check_password(password)])


def part2(input: List[str]) -> int:
    return len([0 for line in input
                for policy, password in [parse_line(PasswordPolicy2, line)]
                if policy.check_password(password)])
