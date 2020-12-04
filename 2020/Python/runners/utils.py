import logging
from typing import Iterable, Iterator, TypeVar
from functools import reduce

T = TypeVar("T")


def product(l: Iterable[T]) -> T:
    return reduce(lambda acc, x: acc * x, l, 1)


def count(items: Iterator[T]) -> int:
    return sum(1 for _ in items)


def isEnabled(level: int) -> bool:
    return logging.getLogger().isEnabledFor(level)
