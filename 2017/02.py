from utils import *

import itertools

def transform_input(i):
    return [lmap(int, line.split('\t')) for line in i.splitlines()]

@check(43074)
def part1(i):
    return sum(max(l) - min(l) for l in i)

@check(280)
def part2(i):
    def division_result(line):
        for a, b in itertools.product(line, line):
            if a != b and a % b == 0: return a // b

    return sum(division_result(l) for l in i)

