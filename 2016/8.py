import re
import itertools
from enum import Enum
from collections import namedtuple
from functools import partial, reduce

width = 50
height = 6


def part1(instructions):
    functions = map(get_function, instructions)
    state = reduce(lambda state, fn: fn(state), functions, set())
    print(len(state))

def part2(instructions):
    functions = map(get_function, instructions)
    state = reduce(lambda state, fn: fn(state), functions, set())

    for y in range(height):
        for x in range(width):
            print("#" if (x, y) in state else " ", end="")
        print()


def rectangle_instruction(state, x, y):
    (xs, ys) = (range(x), range(y))
    added_points = set(itertools.product(xs, ys))
    return state | added_points

def rotate_row_instruction(state, row, amount):
    affected_lights = { l for l in state if l[1] == row }

    new_lights = {
        ((x + amount) % width, y)
        for (x, y)
        in affected_lights
    }

    return (state - affected_lights) | new_lights

def rotate_col_instruction(state, col, amount):
    affected_lights = { l for l in state if l[0] == col }

    new_lights = {
        (x, (y + amount) % height)
        for (x, y)
        in affected_lights
    }

    return (state - affected_lights) | new_lights

def get_function(instruction):
    def make_rectangle_fn(match):
        (x, y) = map(int, match)
        return partial(rectangle_instruction, x=x, y=y)

    def make_rotate_row_fn(match):
        (row, amount) = map(int, match)
        return partial(rotate_row_instruction, row=row, amount=amount)

    def make_rotate_col_fn(match):
        (col, amount) = map(int, match)
        return partial(rotate_col_instruction, col=col, amount=amount)

    fns = [
        ('rect (\d+)x(\d+)', make_rectangle_fn),
        ('rotate row y=(\d+) by (\d+)', make_rotate_row_fn),
        ('rotate column x=(\d+) by (\d+)', make_rotate_col_fn),
    ]

    for (regex, fn) in fns:
        if re.match(regex, instruction):
            groups = re.search(regex, instruction).groups()
            return fn(groups)

def transform_input(challenge_input):
    instruction_strings = challenge_input.splitlines()
    return instruction_strings
