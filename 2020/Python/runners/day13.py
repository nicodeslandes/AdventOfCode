import logging
from typing import List
from logging import debug, info
from runners.utils import isEnabled
import math


def part1(input: List[str]) -> int:
    ts = int(input[0])
    times = [int(l) for l in input[1].split(',') if l != 'x']

    next = ts + 1
    while True:
        for t in times:
            if next % t == 0:
                info("Found %s", (ts, next))
                return t * (next - ts)
        next += 1


def isolve(a, b, c):
    q, r = divmod(a, b)
    if r == 0:
        return([0, c//b])
    else:
        sol = isolve(b, r, c)
    u = sol[0]
    v = sol[1]
    return([v, u - q*v])


def part2(input: List[str]) -> int:
    inputs = input[1].split(',')
    modulos = {}
    for i in range(len(inputs)):
        if inputs[i] != 'x':
            id = int(inputs[i])
            modulos[id] = (id-i) % id

    info("Modulos: %s", modulos)

    prev = None
    max_incr = 0
    offset = 0
    for id, m in modulos.items():
        if prev is not None:
            pid, pm = prev
            debug("Solving %d,%d,%d", pid, -id, m - pm)
            [s1, s2] = isolve(pid, -id, m - pm)
            #t = a . n + b
            b = pid * s1 + pm
            a = pid * -id

            bb = id * s2 + m
            aa = a

            #debug("gcd(%d, %d) = %d", pid, id, math.gcd(pid, id))
            debug("%d - %d*n, %d - %d*n", s1, id, s2, pid)
            debug("Solution: t=%d*n + %d = %d*n + %d", a, b, aa, bb)
            info("Solution: t=%d*n + %d", a, b)
            incr = abs(a)
            if max_incr < incr:
                debug("New increment,offset: %d, %d", incr, offset)
                max_incr = incr
                offset = b
        prev = (id, m)

    ts = offset
    info("Increment,offset: %d, %d", max_incr, ts)
    if ts < 0:
        ts += (1 + (-offset) // max_incr) * max_incr

    info("Increment,offset: %d, %d", max_incr, ts)
    while not all(ts % id == m for id, m in modulos.items()):
        ts += max_incr
        debug("ts: %d", ts)
        # if isEnabled(logging.DEBUG):
        #     debug("%s", [(ts, id, ts % id, m) for id, m in modulos.items()])

    return ts
