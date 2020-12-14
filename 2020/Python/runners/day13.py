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
    """Solves a single diophantine equation of the form ax + by = c
       (Stolen from https://www.math.utah.edu/~carlson/hsp2004/PythonShortCourse.pdf, page 12 😋)
       It returns an array [s1,s2] representing the solution space:
        S = {(s1 + n*b, s2 - n*a), n ∈ ℤ}
    """
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

    modulos_list = list(modulos.items())
    incr = modulos_list[0][0]
    offset = -modulos_list[0][1]
    debug("Current solution space: %d + %dn", offset, incr)

    for i in range(1, len(modulos_list)):
        # Current solution: offset + n*incr
        id, m = modulos_list[i]

        # We need to solve:
        #  t = offset + n1*incr (the current solution space)
        #    = m + id*n2        (the new requirement for the current bus)

        # First, we solve: (1) off + n1*incr = m - id*n2
        #              ie: (2) n1*inc + n2*id = -offset+m

        [s1, s2] = isolve(incr, id, m - offset)

        # Solutions are n1 = s1 + n.id, n2 = s2 - n.inc

        # This means (1) becomes: t = off + (s1 + n.id).inc = m + (-s2 + n.inc).id
        #                       (and btw => off + s1.inc = m - s2.id )
        # so (3) t = (off + s1.inc) + n.(id.inc)
        # (or t = (m - s2.id) + n.(id.inc))

        # In other words, valid values t remain in the form t = a' + n*b',
        # and so we can now update offset and incr from (3):
        offset += s1*incr
        incr *= id

        # We can also bring offset to the first positive congruent value mod incr:
        if offset < 0:
            offset += (1 + (-offset) // incr) * incr
        offset %= incr

        debug("New offset,inc = %d,%d", offset, incr)
        debug("Current solution space: %d + %dn", offset, incr)

    return offset
