from utils import *

import itertools
from collections import defaultdict

@slow
def part1(instruction_strings):
    functions = map(get_function_part1, instruction_strings)
    state = reduce(lambda state, fn: fn(state), functions, set())
    return len(state)

@slow
def part2(instruction_strings):
    functions = map(get_function_part2, instruction_strings)

    state = defaultdict(int)

    for function in functions:
        function(state)

    return sum(state.values())

def turn_on_instruction(x1, y1, x2, y2, state):
    points = get_points(x1, y1, x2, y2)
    return state | points

def turn_off_instruction(x1, y1, x2, y2, state):
    points = get_points(x1, y1, x2, y2)
    return state - points

def toggle_instruction(x1, y1, x2, y2, state):
    points = get_points(x1, y1, x2, y2)
    points_to_remove = set(point for point in points if point in state)
    points_to_add = set(point for point in points if point not in state)

    return state - points_to_remove | points_to_add

def turn_on_brightness_instruction(x1, y1, x2, y2, state):
    points = get_points(x1, y1, x2, y2)

    for point in points:
        state[point] = state[point] + 1

def turn_off_brightness_instruction(x1, y1, x2, y2, state):
    points = get_points(x1, y1, x2, y2)

    for point in points:
        state[point] = max(state[point] - 1, 0)

def toggle_brightness_instruction(x1, y1, x2, y2, state):
    points = sorted(get_points(x1, y1, x2, y2))

    for point in points:
        state[point] = state[point] + 2

def get_points(x1, y1, x2, y2):
    return set(itertools.product(range(x1, x2+1), range(y1, y2+1)))

def get_function_part1(instruction):
    fns = [
        ('turn off (\d+),(\d+) through (\d+),(\d+)', turn_off_instruction),
        ('turn on (\d+),(\d+) through (\d+),(\d+)', turn_on_instruction),
        ('toggle (\d+),(\d+) through (\d+),(\d+)', toggle_instruction),
    ]

    for (regex, fn) in fns:
        if re.match(regex, instruction):
            groups = re.search(regex, instruction).groups()
            groups = map(int, groups)
            return partial(fn, *groups)

def get_function_part2(instruction):
    fns = [
        ('turn off (\d+),(\d+) through (\d+),(\d+)', turn_off_brightness_instruction),
        ('turn on (\d+),(\d+) through (\d+),(\d+)', turn_on_brightness_instruction),
        ('toggle (\d+),(\d+) through (\d+),(\d+)', toggle_brightness_instruction),
    ]

    for (regex, fn) in fns:
        if re.match(regex, instruction):
            groups = re.search(regex, instruction).groups()
            groups = map(int, groups)
            return partial(fn, *groups)

def transform_input(challenge_input): return challenge_input.splitlines()
