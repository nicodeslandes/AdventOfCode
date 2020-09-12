from utils.log_init import set_log_level
import logging
from logging import debug, info, warning, error
import argparse
import importlib
import re
import requests
import os
import sys


class PuzzleRunner:
    def __init__(self):
        self.data_loader = PuzzleDataLoader()

    def run_puzzle(self, day, part=None):
        debug("Starting execution of day %d", day)
        module = f'runners.day{day}'
        self.run_puzzle_module(module, part)

    def run_puzzle_module(self, module, part):
        if m := re.match(r"runners.day(\d+)", module):
            day = int(m.group(1))
        else:
            raise Exception(f"Invalid module name: {module}")

        info("Executing puzzle for day %d", day)
        if part is not None:
            debug("Only executing part %d", part)

        input = self.data_loader.get_input_file(day)
        debug("Loading module %s", module)
        day_module = importlib.import_module(module)
        if (part is None or part == 1) and "part1" in day_module.__dict__:
            result = day_module.part1(input)
            print("Day {} part 1: {}".format(day, result))

        if (part is None or part == 2) and "part2" in day_module.__dict__:
            result = day_module.part2(input)
            print("Day {} part 2: {}".format(day, result))


class PuzzleDataLoader:
    def get_input_file(self, day):
        # Try to load the cached copy
        input_cache_dir = f".data/day{day}"
        input_cache_name = f"{input_cache_dir}/input.txt"
        if os.path.exists(input_cache_name):
            with open(input_cache_name) as f:
                return f.readlines()

        # If there's no local copy, download it
        cookie = self.load_cookie()
        input = requests.get(f"https://adventofcode.com/2019/day/{day}/input",
                             cookies=dict(session=cookie)).text

        self.save_input(input, input_cache_dir, input_cache_name)
        return input.splitlines()

    def save_input(self, input, input_cache_dir, input_cache_name):
        if not os.path.exists(input_cache_dir):
            os.mkdir(input_cache_dir)

        with open(input_cache_name, "w") as f:
            f.write(input)

    def load_cookie(self):
        cookie_file_dir = f"{sys.path[0]}/.data"
        if not os.path.exists(cookie_file_dir):
            os.mkdir(cookie_file_dir)

        with open(f"{cookie_file_dir}/cookie.txt") as cookie_file:
            return cookie_file.readline().rstrip()


def main():
    args = parse_args()
    setup_log_level(args.verbosity)

    info("Hello, welcome to Advent Of Code 2019")
    runner = PuzzleRunner()
    if args.list:
        import runners
        for m in runners.__all__:
            print("Available day:", m)
    elif args.day is not None:
        runner.run_puzzle(args.day, args.part)
    elif args.run_all:
        info("Running all puzzles")
        import runners
        for day_module in runners.__all__:
            runner.run_puzzle_module(f"runners.{day_module}", args.part)
    else:
        raise "Invalid arguments"


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbosity", action="count", default=0,
                        help="increase output verbosity")
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "-l", "--list", help="list available days", action="store_true")
    group.add_argument("-r", "--run", help="run the puzzle for a specific day",
                       type=int, dest="day")
    group.add_argument("-a", "--run-all", help="run all puzzles",
                       action="store_true")
    parser.add_argument(
        "-p", "--part", choices=[1, 2], type=int, help="only run a single part of the puzzle(s)")

    return parser.parse_args()


def setup_log_level(verbosity):
    if verbosity == 0:
        set_log_level(logging.WARNING)
    elif verbosity == 1:
        set_log_level(logging.INFO)
    else:
        set_log_level(logging.DEBUG)

    debug("Verbosity level: %d", verbosity)


if __name__ == '__main__':
    main()
