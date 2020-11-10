from argparse import Namespace
from typing import Any, List, Optional
from utils.log_init import set_log_level
import logging
from logging import debug, info, warning, error
import argparse
import importlib
import re
import requests
import os
import sys
import time


class Options:
    useTestFile: int

    def __init__(self, useTestFile: int):
        self.useTestFile = useTestFile


class PuzzleRunner:
    def __init__(self, options: Options):
        self.data_loader = PuzzleDataLoader()

    def run_puzzle(self, day, part, test):
        debug("Starting execution of day %d", day)
        module = f'runners.day{day}'
        self.run_puzzle_module(module, part, test)

    def run_puzzle_module(self, module: str, part: Optional[int], test_file: Optional[int]):
        if m := re.match(r"runners.day(\d+)", module):
            day = int(m.group(1))
        else:
            raise Exception(f"Invalid module name: {module}")

        debug("Loading module %s", module)
        day_module: Any = importlib.import_module(module)

        if test_file == 0:
            self.run_all_tests(day_module, day, part)
        else:
            self._run_puzzle(day_module, day, part, test_file)

    def run_all_tests(self, day_module: Any, day: int, part: Optional[int]):
        def try_run_part(part: int):
            if f"part{part}" in day_module.__dict__:
                test = 1
                while True:
                    test_file = self.data_loader.get_input_file_path(day, part, test)
                    if os.path.exists(test_file):
                        self._run_puzzle(day_module, day, part, test)
                        test += 1
                    else:
                        break
        
        debug("Running all tests for day %d", day)
        if part is None or part == 1: try_run_part(1)
        if part is None or part == 2: try_run_part(2)

    def _run_puzzle(self, day_module: Any, day: int, part: Optional[int], test: Optional[int]):
        info("Executing puzzle for day %d%s%s", day, f" part {part}" if part else "", f" test {test}" if test else "")
        if part is not None:
            debug("Only executing part %d", part)

        if (part is None or part == 1) and "part1" in day_module.__dict__:
            def run_part1(input): return day_module.part1(input, is_test = test is not None)
            self.run(day, 1, test, run_part1)

        if (part is None or part == 2) and "part2" in day_module.__dict__:
            def run_part2(input): return day_module.part2(input)
            self.run(day, 2, test, run_part2)

    def run(self, day, part, test, func):
        puzzle_data = self.data_loader.get_puzzle_data(day, part, test)
        input = puzzle_data.get_data()
        expected_result = puzzle_data.get_expected_result()

        start = time.perf_counter()
        result = func(input)

        comparison_result = ""
        if expected_result is not None:
            if expected_result == str(result):
                comparison_result = " ✔️ "
            else:
                comparison_result = f" ❌ ({expected_result} expected)"

        elapsed_ms = (time.perf_counter() - start) * 1000
        print("Day {} part {}{}: {}{} - {:,} ms".format(
            day, part, f' test {test}' if test else '', result, comparison_result, int(elapsed_ms)))

class PuzzleData:
    filename: str
    expected_result: Optional[str]

    def __init__(self, filename: str, is_test_file: bool):
        self.filename = filename
        self.is_test_file = is_test_file

    def get_data(self) -> List[str]:
        with open(self.filename) as f:
            lines = f.readlines()
            return lines if not self.is_test_file else lines[2:]
    
    def get_expected_result(self) -> Optional[str]:
        if not self.is_test_file:
            return None

        with open(self.filename) as f:
            first_line = f.readline().strip()
            if not first_line.startswith("Result: "):
                raise Exception(f"Invalid test file {self.filename}; it should start with 'Result: '")

            return first_line.replace("Result: ", "")


class PuzzleDataLoader:
    def get_cache_dir(self, day: int) -> str:
        return f".data/day{day}"

    def get_input_file_path(self, day: int, part: int, test: Optional[int]) -> str:
        input_cache_dir = self.get_cache_dir(day)
        file_name = "input.txt" if not test else f"test{test}_part{part}.txt"
        input_file = f"{input_cache_dir}/{file_name}"
        debug("Input file: %s", input_file)
        return input_file

    def get_puzzle_data(self, day: int, part: int, test: Optional[int]) -> PuzzleData:
        # Try to load the cached copy
        input_cache_name = self.get_input_file_path(day, part, test)
        if os.path.exists(input_cache_name):
            return PuzzleData(input_cache_name, test != None)

        if test:
            content = self.read_test_file(input_cache_name, test)
        else:
            # If there's no local copy, download it
            cookie = self.load_cookie()
            content = requests.get(f"https://adventofcode.com/2019/day/{day}/input",
                                   cookies=dict(session=cookie)).text

        self.save_input(content, self.get_cache_dir(day), input_cache_name)
        return PuzzleData(input_cache_name, test != None)

    def read_test_file(self, filename: str, test_file: int) -> str:
        info("File %s not found; requesting content from user", filename)
        print("Please enter the input for test %d; end with an empty line" %
                test_file)

        content = ""
        while True:
            try:
                line = input()
                if line == "":
                    break
            except EOFError:
                break

            if content != "":
                content += "\n"
            content += line

        result = input("Expected result: ")
        return f"Result: {result}\nInput:\n{content}"

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
    options = Options(useTestFile=args.test)
    runner = PuzzleRunner(options)
    if args.list:
        import runners
        for m in runners.__all__:
            print("Available day:", m)
    elif args.day is not None:
        runner.run_puzzle(args.day, args.part, args.test)
    elif args.run_all:
        info("Running all puzzles")
        import runners
        for day_module in runners.__all__:
            runner.run_puzzle_module(f"runners.{day_module}", args.part, args.test)
    else:
        raise Exception("Invalid arguments")


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
    parser.add_argument(
        "-t", "--t", type=int, help="use test input TEXT.txt", dest="test", const=0, nargs="?")

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
