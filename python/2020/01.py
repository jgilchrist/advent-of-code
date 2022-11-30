from utils import *

import itertools
import math

@check(55776)
def part1(i):
    pairs = itertools.combinations(i, 2)

    for combinations in pairs:
        if sum(combinations) == 2020:
            return math.prod(combinations)

@check(223162626)
def part2(i):
    pairs = itertools.combinations(i, 3)

    for combinations in pairs:
        if sum(combinations) == 2020:
            return math.prod(combinations)

def transform_input(i):
    return [int(line) for line in i.splitlines()]
