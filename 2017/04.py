from utils import *

def part1(i):
    return sum(len(set(words)) == len(words) for words in i)

def part2(i):
    i = [["".join(sorted(word)) for word in line] for line in i]
    return sum(len(set(words)) == len(words) for words in i)

def transform_input(i):
    return [line.split() for line in i.splitlines()]
