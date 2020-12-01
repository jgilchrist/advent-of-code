from utils import *

from itertools import cycle

def transform_input(i):
    return [int(line) for line in i.splitlines()]

@check(400)
def part1(i):
    return sum(i)

@check(232)
def part2(i):
    numbers = cycle(i)

    seen_frequencies = {0}
    frequency = 0

    for n in numbers:
        frequency += n

        if frequency in seen_frequencies:
            return frequency

        seen_frequencies.add(frequency)
