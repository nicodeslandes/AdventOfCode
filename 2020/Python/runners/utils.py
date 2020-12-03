from typing import Iterable, TypeVar
from functools import reduce

T = TypeVar("T")


def product(l: Iterable[T]) -> T:
    return reduce(lambda acc, x: acc * x, l, 1)
