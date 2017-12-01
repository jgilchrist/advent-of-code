from utils import *

from itertools import cycle

def part1(i):
    return sum_of_matching_pairs(window(i+i[0], 2))

def part2(i):
    rotated_i = cycle(i)
    for _ in range(len(i)//2):
        next(rotated_i)

    return sum_of_matching_pairs(zip(i, rotated_i))

def sum_of_matching_pairs(list_of_pairs):
    return sum(int(a) for (a, b) in list_of_pairs if a == b)
