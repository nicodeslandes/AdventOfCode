from typing import List, Set, Tuple
import re

from logging import debug


class BagDescriptor:
    inner_bags: dict[str, int]

    def __init__(self, bag_type: str) -> None:
        self.inner_bags = {}
        self.bag_type = bag_type


class BagType:
    inner_bags: dict[str, Tuple[int, 'BagType']]
    parent_bags: dict[str, Tuple[int, 'BagType']]

    def __init__(self, bag_type: str) -> None:
        self.inner_bags = {}
        self.parent_bags = {}
        self.bag_type = bag_type


def parse_bag_types(input: List[str]) -> dict[str, BagType]:
    bags: dict[str, BagDescriptor] = {}
    for line in input:
        [bag_type, content] = line.split(' contain ')
        bag_type = bag_type.removesuffix(' bags')
        content = content.removesuffix('.')
        desc = BagDescriptor(bag_type)
        if content != "no other bags":
            content_bags = content.split(", ")
            for b in content_bags:
                m = re.match(r"(\d+) (.+) bags?", b)
                if m:
                    count = int(m.group(1))
                    type = m.group(2)
                    desc.inner_bags[type] = count

        bags[bag_type] = desc

    debug("Bag descriptors: %s", bags)

    bag_types: dict[str, BagType] = {}
    for d in bags:
        debug("Adding '%s'", d)
        bag_types[d] = BagType(d)

    for d, desc in bags.items():
        for inner, count in desc.inner_bags.items():
            bag_types[d].inner_bags[inner] = count, bag_types[inner]
            bag_types[inner].parent_bags[d] = count, bag_types[d]

    return bag_types


def part1(input: List[str]) -> int:
    bag_types = parse_bag_types(input)

    # Walk through all parents of shiny gold, recursively
    parents: Set[str] = set()

    def add_parents(bt: BagType):
        for p, parent in bt.parent_bags.items():
            parents.add(p)
            add_parents(parent[1])

    add_parents(bag_types["shiny gold"])
    return len(parents)


def part2(input: List[str]) -> int:
    bag_types = parse_bag_types(input)

    # Walk through all inner bags of shiny gold, recursively
    count = 0

    def add_children(bt: BagType, multiplier: int = 1):
        for p, (c, child) in bt.inner_bags.items():
            nonlocal count
            count += c * multiplier
            add_children(child, multiplier * c)

    add_children(bag_types["shiny gold"])
    return count
