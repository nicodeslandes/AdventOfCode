from utils.log_init import set_log_level
import logging
from logging import debug, info, warning, error
import argparse


def main():
    set_log_level(logging.INFO)
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--log", help="set the log level",
                        type=str, choices=['debug', 'info', 'warn'])
    args = parser.parse_args()

    if args.log is not None:
        info("Log level: %s", args.log)

    info("Hello, welcome to Advent Of Code 2019")
    debug("Starting")


if __name__ == '__main__':
    main()
