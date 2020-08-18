from utils.log_init import set_log_level
import logging
from logging import debug, info, warning, error


def main():
    set_log_level(logging.INFO)
    info("Hello, welcome to Advent Of Code 2019")
    debug("Starting")


if __name__ == '__main__':
    main()
