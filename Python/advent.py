from utils.log_init import set_log_level
import logging
from logging import debug, info, warning, error
import argparse


def main():
    args = parse_args()
    setup_log_level(args.log)

    info("Hello, welcome to Advent Of Code 2019")
    debug("Starting execution of day %d", args.day)


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("day", help="which day to run", type=int)
    parser.add_argument("-l", "--log", help="set the log level",
                        type=str, choices=['debug', 'info', 'warn'])
    return parser.parse_args()


def setup_log_level(level):
    if level is None:
        level = 'info'

    if level == 'debug':
        set_log_level(logging.DEBUG)
    elif level == 'info':
        set_log_level(logging.INFO)
    elif level == 'warn':
        set_log_level(logging.WARN)
    else:
        raise f'Incorrect log level: {level}'

    debug("Log level: %s", level)


if __name__ == '__main__':
    main()
