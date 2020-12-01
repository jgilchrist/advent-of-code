from utils import *

@check(74)
def part1(challenge_input):
    up_floors = challenge_input.count('(')
    down_floors = challenge_input.count(')')

    return up_floors - down_floors

@check(1795)
def part2(challenge_input):
    floor = 0
    for i, c in enumerate(challenge_input, start=1):
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1

        if floor == -1:
            return i
            break
