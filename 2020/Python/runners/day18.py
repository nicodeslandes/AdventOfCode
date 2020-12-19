from typing import List
import re


def execute1(expr: str) -> int:
    if m := re.search(r"\(([^\(\)]+)\)", expr):
        sub = m.group(1)
        return execute1(expr.replace(f"({sub})", f"{execute1(sub)}", 1))

    if m := re.search(r"(\d+) ([+*]) (\d+)", expr):
        a = int(m.group(1))
        op = m.group(2)
        b = int(m.group(3))
        r = a + b if op == '+' else a * b
        return execute1(expr.replace(f"{a} {op} {b}", f"{r}", 1))

    return int(expr)


def execute2(expr: str) -> int:
    if m := re.search(r"\(([^\(\)]+)\)", expr):
        sub = m.group(1)
        return execute2(expr.replace(f"({sub})", f"{execute2(sub)}", 1))

    if m := re.search(r"(\d+) \+ (\d+)", expr):
        a = int(m.group(1))
        b = int(m.group(2))
        return execute2(expr.replace(f"{a} + {b}", f"{a+b}", 1))

    if m := re.search(r"(\d+) \* (\d+)", expr):
        a = int(m.group(1))
        b = int(m.group(2))
        return execute2(expr.replace(f"{a} * {b}", f"{a*b}", 1))

    return int(expr)


def part1(input: List[str]) -> int:
    return sum(execute1(e) for e in input)


def part2(input: List[str]) -> int:
    return sum(execute2(e) for e in input)
