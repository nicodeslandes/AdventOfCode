from typing import List, Optional
from logging import debug, info
import os
import requests
import sys

from requests.models import Response


class PuzzleData:
    filename: str
    expected_result: Optional[str]

    def __init__(self, filename: str, is_test_file: bool):
        self.filename = filename
        self.is_test_file = is_test_file

    def get_data(self) -> List[str]:
        with open(self.filename) as f:
            lines = f.readlines()
            if self.is_test_file:
                lines = lines[2:]
            return list(map(str.strip, lines))

    def get_expected_result(self) -> Optional[str]:
        if not self.is_test_file:
            return None

        with open(self.filename) as f:
            first_line = f.readline().strip()
            if not first_line.startswith("Result: "):
                raise Exception(
                    f"Invalid test file {self.filename}; it should start with 'Result: '")

            return first_line.replace("Result: ", "")


class PuzzleDataLoader:
    def get_cache_dir(self, day: int) -> str:
        return f"data/day{day}"

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
            response: Response = requests.get(f"https://adventofcode.com/2020/day/{day}/input",
                                              cookies=dict(session=cookie))
            if not response.ok:
                raise Exception(
                    f"Error while downloading input for puzzle {day}: {response.text}")
            content = response.text

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

    def save_input(self, input: str, input_cache_dir: str, input_cache_name: str) -> None:
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
