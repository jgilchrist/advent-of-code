from utils import *

@check(6297)
def part1(i):
    i = [set.union(*lines) for lines in i]
    return sum(len(union) for union in i)

@check(3158)
def part2(i):
    i = [set.intersection(*lines) for lines in i]
    return sum(len(intersection) for intersection in i)

def transform_input(i):
    i = [line.splitlines() for line in i.split('\n\n')]
    return [[set(line) for line in lines] for lines in i]
