from typing import List, Set
from logging import debug, info

passport_fields = set([
    'byr',
    'iyr',
    'eyr',
    'hgt',
    'hcl',
    'ecl',
    'pid',
    # 'cid'
])


def part1(input: List[str]) -> int:
    def read_passports():
        current = {}
        for line in input:
            if line == "":
                yield current
                current.clear()
            else:
                current |= dict([(key, val) for kvp in line.split(' ')
                                 for [key, val] in [kvp.split(':')]])
        if len(current) != 0:
            yield current

    def is_valid(passport: dict):
        missing_keys = set(passport_fields)
        info("Checking passport %s", passport)
        debug("Missing keys: %s", missing_keys)
        for key in passport.keys():
            missing_keys.discard(key)

        debug("Missing keys: %s", missing_keys)
        return len(missing_keys) == 0

    for passport in read_passports():
        info("Passport: %s: %s", passport, is_valid(passport))

    return sum([int(is_valid(p)) for p in read_passports()])

# def part2(input: List[str]) -> int:
