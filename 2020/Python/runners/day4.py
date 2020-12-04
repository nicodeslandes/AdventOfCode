from typing import List, Set
from logging import debug, info
import re


def check_number(min: int, max: int):
    def check(value):
        if not re.match(r"^\d+$", value):
            return False
        x = int(value)
        return x >= min and x <= max

    return check


def check_height():
    def check(value):
        m = re.match(r"^(\d+)(cm|in)$", value)
        if not m:
            return False
        value = int(m.group(1))
        if m.group(2) == "cm":
            return value >= 150 and value <= 193
        if m.group(2) == "in":
            return value >= 59 and value <= 76
        return False

    return check


def check_regex(regex):
    return lambda value: re.fullmatch(regex, value) != None


passport_fields = {
    'byr': check_number(1920, 2002),
    'iyr': check_number(2010, 2020),
    'eyr': check_number(2010, 2030),
    'hgt': check_height(),
    'hcl': check_regex(r"#[0-9a-f]{6}"),
    'ecl': check_regex(r"amb|blu|brn|gry|grn|hzl|oth"),
    'pid': check_regex(r"[0-9]{9}"),
    # 'cid'
}


def read_passports(input):
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


def is_valid(passport: dict[str, str]):
    missing_keys = set(passport_fields.keys())
    debug("Checking passport %s", passport)
    debug("Missing keys: %s", missing_keys)
    for key, value in passport.items():
        validator = passport_fields.get(key)
        if validator and validator(value):
            missing_keys.discard(key)

    debug("Missing keys: %s", missing_keys)
    return len(missing_keys) == 0


def part1(input: List[str]) -> int:
    for passport in read_passports(input):
        info("Passport: %s: %s", passport, is_valid(passport))

    return sum([int(is_valid(p)) for p in read_passports(input)])


def part2(input: List[str]) -> int:
    for passport in read_passports(input):
        info("Passport: %s: %s", passport, is_valid(passport))

    return sum([int(is_valid(p)) for p in read_passports(input)])
