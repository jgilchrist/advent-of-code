from enum import IntEnum
from utils import direction_offset, lmap, tuple_add

def part1(instructions):
    location = (1, 1)

    code = []

    for instruction_list in instructions:
        for direction in instruction_list:
            location = move(location, direction)

        number = location_to_number(location)
        code.append(number)

    print("".join(map(str, code)))

def part2(instructions):
    state = (1, 1)
    pass


def transform_input(challenge_input):
    def parse_instruction(i):
        return {
            'U': Direction.Up,
            'D': Direction.Down,
            'L': Direction.Left,
            'R': Direction.Right,
        }[i]

    lines = challenge_input.splitlines()
    return [lmap(parse_instruction, line) for line in lines]


class Direction(IntEnum):
    Up = 1
    Right = 2
    Down = 3
    Left = 4

def move(location, direction):
    offset = direction_offset(direction)
    new_location = tuple_add(location, offset)

    if not is_valid_location(new_location):
        return location

    return new_location

def is_valid_location(location):
    (x, y) = location
    return x >= 0 and x <= 2 and y >= 0 and y <= 2

def location_to_number(location):
    (x, y) = location
    return [[1, 2, 3], [4, 5, 6], [7, 8, 9]][y][x]
