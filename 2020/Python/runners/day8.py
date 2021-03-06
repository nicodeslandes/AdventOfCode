import logging
from typing import Iterable, List, Tuple
from logging import debug, info
from runners.utils import isEnabled


def read_instructions(input: List[str]) -> Iterable[Tuple[str, int]]:
    for line in input:
        [instr, arg] = line.split(' ')
        yield instr, int(arg)


def run(code: List[Tuple[str, int]]) -> Tuple[int, bool]:
    ips = set()
    ip, acc = 1, 0
    while True:
        inst, arg = code[ip-1]
        debug("Runnning code at %d: %s %d; acc: %d", ip, inst, arg, acc)
        if inst == "nop":
            ip += 1
        elif inst == "acc":
            acc += arg
            ip += 1
        else:
            ip += arg

        if ip in ips:
            return acc, False

        if ip == len(code) + 1:
            return acc, True

        ips.add(ip)


def part1(input: List[str]) -> int:
    code = list(read_instructions(input))
    output, _ = run(code)
    return output


def part2(input: List[str]) -> int:
    code = list(read_instructions(input))

    for i in range(len(code)):
        patched = list(code)
        inst, arg = patched[i]
        if inst == "nop":
            patched[i] = "jmp", arg
        elif inst == "jmp":
            patched[i] = "nop", arg
        else:
            continue

        if isEnabled(logging.DEBUG):
            debug("Code: \n%s", "\n".join(str(x) for x in patched))
        output, finished = run(patched)
        info("Result: %s", (output, finished))
        if finished:
            return output

    raise Exception("Failed!!")
