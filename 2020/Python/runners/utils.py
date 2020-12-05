import logging
from typing import Callable, Iterable, Iterator, Optional, Tuple, TypeVar
from functools import reduce

T = TypeVar("T")
TNumeric = TypeVar("TNumeric", int, float)


def product(l: Iterable[TNumeric]) -> TNumeric:
    return reduce(lambda acc, x: acc * x, l, 1)


def count(items: Iterator[T]) -> int:
    return sum(1 for _ in items)


def isEnabled(level: int) -> bool:
    return logging.getLogger().isEnabledFor(level)


def zipwithnext(src: Iterable[T]) -> Iterable[Tuple[T, T]]:
    iter = src.__iter__()
    try:
        prev = iter.__next__()
        while True:
            current = iter.__next__()
            yield prev, current
            prev = current
    except StopIteration:
        pass


def findfirst(src: Iterable[T], pred: Callable[[T], bool]) -> Optional[T]:
    for item in src:
        if pred(item):
            return item
    return None
