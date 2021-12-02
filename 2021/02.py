from utils import *

def part1(i):
    depth = 0
    pos = 0

    for (command, n) in i:
        if command == 'forward':
            pos += n
        elif command == 'up':
            depth -= n
        elif command == 'down':
            depth += n

    return pos * depth


def part2(i):
    depth = 0
    pos = 0
    aim = 0

    for (command, n) in i:
        if command == 'forward':
            pos += n
            depth += aim * n
        elif command == 'up':
            aim -= n
        elif command == 'down':
            aim += n

    return pos * depth

def transform_input(i):
    lines = i.splitlines()
    lines = [l.split() for l in lines]
    return [(command, int(n)) for (command, n) in lines]
