from utils import *

from itertools import cycle

def part1(i):
    return sum_of_matching_pairs(i, 1)

def part2(i):
    return sum_of_matching_pairs(i, len(i)//2)

def rotate_by(i, n):
    i = cycle(i)
    take(n, i)
    return i

def sum_of_matching_pairs(seq, n):
    return sum(int(a) for (a, b) in zip(seq, rotate_by(seq, n)) if a == b)
