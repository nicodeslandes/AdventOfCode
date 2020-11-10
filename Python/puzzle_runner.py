
from puzzle_data import PuzzleDataLoader
from typing import Any, Optional
from logging import debug, info, warning, error
import importlib
import re
import os
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

        def run_part_if_present(part: int):
            func = day_module.__dict__.get(f'part{part}')
            if func:
                def run(input): return func(input, is_test = test is not None)
                self.run(day, part, test, run)

        if (part is None or part == 1): run_part_if_present(1)
        if (part is None or part == 2): run_part_if_present(2)

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
