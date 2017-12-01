from utils import *

from itertools import cycle

def part1(i):
    return sum_of_matching_pairs(i, rotate_by(i, 1))

def part2(i):
    return sum_of_matching_pairs(i, rotate_by(i, len(i)//2))

def rotate_by(i, n):
    i = cycle(i)
    for _ in range(n):
        next(i)
    return i

def sum_of_matching_pairs(seq1, seq2):
    return sum(int(a) for (a, b) in zip(seq1, seq2) if a == b)
