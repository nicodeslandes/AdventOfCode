from puzzle_runner import Options, PuzzleRunner
from utils.log_init import set_log_level
import logging
from logging import debug, info
import argparse

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
    elif args.day:
        runner.run_puzzle(args.day, args.part, args.test)
    elif args.run_all:
        runner.run_all_puzzles(args.part, args.test)
    elif args.add_day:
        runner.add_day(args.add_day)
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
    group.add_argument("--add", type=int, metavar="DAY", dest="add_day", help="add a new empty solution for the day")
    parser.add_argument(
        "-p", "--part", choices=[1, 2], type=int, help="only run a single part of the puzzle(s)")
    parser.add_argument(
        "-t", "--t", type=int, help="use test input TEXT.txt", dest="test", const=0, nargs="?")

    return parser.parse_args()


def setup_log_level(verbosity: int):
    if verbosity == 0:
        set_log_level(logging.WARNING)
    elif verbosity == 1:
        set_log_level(logging.INFO)
    else:
        set_log_level(logging.DEBUG)

    debug("Verbosity level: %d", verbosity)


if __name__ == '__main__':
    main()
