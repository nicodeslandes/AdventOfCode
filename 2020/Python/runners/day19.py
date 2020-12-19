from abc import abstractproperty
from typing import Callable, Dict, List, Tuple
from logging import debug, info
from runners.utils import count
import re


class Context:
    def __init__(self, input: str):
        self.input = input
        self.index = 0

    def clone(self) -> "Context":
        c = Context(self.input)
        c.index = self.index
        return c

    def __repr__(self) -> str:
        return f"({self.input}, {self.index}) ({'<eof>' if self.index >= len(self.input) else self.input[self.index]})"


Rule = Callable[[Context], Tuple[bool, int]]


def parse_rules_old(rule_strings: List[str]) -> Dict[int, Rule]:
    rules: Dict[int, Rule] = {}
    for rule_string in rule_strings:
        [rule_index, s] = rule_string.split(": ")
        rule_index = int(rule_index)
        if s.startswith('"'):
            matched_ch = s[1]

            def match(s: Context, rule_index=rule_index, matched_ch=matched_ch):
                r = s.index < len(s.input) and s.input[s.index] == matched_ch
                debug("Matching rule [%d] ('%s'); context: %s -> %s",
                      rule_index, matched_ch, s, r)
                s.index += 1
                return r, 1

            rules[rule_index] = lambda c, match=match: match(c)
        else:
            or_rules_strings = s.split(' | ')
            or_rules = []
            for and_str in or_rules_strings:
                sub_rules = list(map(int, and_str.split(' ')))

                def and_rule(s: Context, sub_rules=sub_rules):
                    debug("Matching 'and' rules %s; context: %s", sub_rules, s)
                    count = 0
                    for i in sub_rules:
                        r = rules[i]
                        [matched, c] = r(s)
                        if not matched:
                            debug("Rules %s result: %s", sub_rules, False)
                            return False, 0
                        else:
                            count += c

                    debug("Rules %s result: %s,%d", sub_rules, True, count)
                    return True, count

                or_rules.append(and_rule)

            def match(c: Context, rule_index=rule_index, or_rules=or_rules):
                debug("Matching rule [%d]; context: %s", rule_index, c)
                for arg in or_rules:
                    clone = c.clone()
                    r, c = arg(clone)
                    if r:
                        debug("Rules [%d] result: %s,%d", rule_index, r, c)
                        return r, c

                debug("Rules [%d] result: %s,%d", rule_index, False, 0)
                return False, 0

            rules[rule_index] = lambda c, match=match: match(c)

    return rules


def parse_rules(raw_rules: Dict[int, str] = {}) -> Dict[int, str]:
    rules: Dict[int, str] = {}

    def get_rule(index: int) -> str:
        r = rules.get(index)
        if r:
            return r
        else:
            r = parse_rule(index, raw_rules[index])
            rules[index] = r
            return r

    def parse_rule(index, rule_string: str) -> str:
        debug("Parsing rule %s", rule_string)

        if (index, rule_string) == (8, "42 | 42 8"):
            return f"({get_rule(42)})+"
        elif (index, rule_string) == (11, "42 31 | 42 11 31"):
            r1 = f"({get_rule(42)})"
            r2 = f"({get_rule(31)})"
            # Cheat!!! Only allow up to 10 repetition for rule 11
            return "(" + "|".join(r1*i + r2*i for i in range(1, 10)) + ")"

        if rule_string.startswith('"'):
            debug("Rule %s -> %s", rule_string, rule_string[1])
            return rule_string[1]
        else:
            or_rules_strings = rule_string.split(' | ')
            if len(or_rules_strings) > 1:
                r = "(" + ")|(".join(parse_rule(-1, clause)
                                     for clause in or_rules_strings) + ")"
            else:
                r = "".join("(" + get_rule(int(r)) +
                            ")" for r in rule_string.split(' '))

            debug("Rule %s -> %s", rule_string, r)
            return r

    for rule_index, rule_string in raw_rules.items():
        get_rule(rule_index)

    return rules


def match_rule(input: str, rules, rule):
    c = Context(input)
    r, c = rules[rule](c)
    info("Matching rule %d for string %s: %s; context: %s", rule, input, r, c)
    return r and c == len(c.input)


def part1_old(input: List[str]) -> int:
    parsing_rules = True
    rule_strings = []
    inputs = []
    for line in input:
        if parsing_rules:
            if line == "":
                parsing_rules = False
                continue
            rule_strings.append(line)
        else:
            inputs.append(line)

    rules = parse_rules_old(rule_strings)
    return count(i for i in inputs if match_rule(i, rules, 0))


def parse_input(input: List[str]) -> Tuple[List[str], Dict[int, str]]:
    parsing_rules = True
    rule_strings = []
    raw_rules = {}
    inputs = []
    for line in input:
        if parsing_rules:
            if line == "":
                parsing_rules = False
                continue
            rule_strings.append(line)
        else:
            inputs.append(line)

    for rule_string in rule_strings:
        [rule_index, s] = rule_string.split(": ")
        rule_index = int(rule_index)
        raw_rules[rule_index] = s

    return inputs, raw_rules


def part1(input: List[str]) -> int:
    inputs, raw_rules = parse_input(input)
    rules = parse_rules(raw_rules)
    return count(i for i in inputs if re.fullmatch(rules[0], i))


def part2(input: List[str]) -> int:
    inputs, raw_rules = parse_input(input)
    raw_rules[8] = "42 | 42 8"
    raw_rules[11] = "42 31 | 42 11 31"
    rules = parse_rules(raw_rules)
    return count(i for i in inputs if re.fullmatch(rules[0], i))
