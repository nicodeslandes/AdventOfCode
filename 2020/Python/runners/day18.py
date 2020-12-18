from typing import List
import re
from logging import debug, info


def execute(expr: str, level=0) -> int:
    if level == 0:
        info("Evaluating %s", expr)
    else:
        debug("%sEvaluating %s", " "*level, expr)

    if m := re.search(r"\(([^\(\)]+)\)", expr):
        sub = m.group(1)
        return execute(expr.replace(f"({sub})", f"{execute(sub, level + 1)}", 1), level + 1)

    if m := re.search(r"(\d+) ([+*]) (\d+)", expr):
        a = int(m.group(1))
        op = m.group(2)
        b = int(m.group(3))
        r = a + b if op == '+' else a * b
        return execute(expr.replace(f"{a} {op} {b}", f"{r}", 1), level + 1)

    # if m := re.search(r"(\d+) \* (\d+)", expr):
    #     a = int(m.group(1))
    #     b = int(m.group(2))
    #     return execute(expr.replace(f"{a} * {b}", f"{a*b}"))

    if level == 0:
        info("Result: %s", expr)
    return int(expr)


def part1(input: List[str]) -> int:
    return sum(execute(e) for e in input)

# def part2(input: List[str]) -> int:
