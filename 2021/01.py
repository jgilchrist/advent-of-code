from utils import *

def part1(i):
    diffs = [d2 - d1 for (d1, d2) in window(i, n=2)]
    return len([d for d in diffs if d > 0])

def part2(i):
    sums = [sum([x, y, z]) for (x, y, z) in window(i, n=3)]
    diffs = [d2 - d1 for (d1, d2) in window(sums, n=2)]
    return len([d for d in diffs if d > 0])

def transform_input(i):
    return lmap(int, i.splitlines())

