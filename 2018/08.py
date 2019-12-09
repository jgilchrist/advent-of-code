from utils import *

def count(i):
    number_of_children = next(i)
    number_of_metadata = next(i)

    return sum(count(i) for _ in range(number_of_children)) + sum(next(i) for _ in range(number_of_metadata))

def part1(i):
    return count(iter(i))
    

def part2(i):
    pass

def transform_input(i):
    return list(map(int, i.split(' ')))
