from enum import IntEnum
from utils import direction_offset, lmap, tuple_add, concat

def part1(instructions):
    location = (1, 1)

    board = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]

    return get_code(location, board, instructions)


def part2(instructions):
    location = (0, 2)

    board = [
        [None, None,    1, None, None],
        [None,    2,    3,    4, None],
        [   5,    6,    7,    8,    9],
        [None,  'A',  'B',  'C', None],
        [None, None,  'D', None, None],
    ]

    return get_code(location, board, instructions)


def get_code(location, board, instructions):
    code = []

    for instruction_list in instructions:
        for direction in instruction_list:
            location = move(location, direction, board)

        (x, y) = location
        number = board[y][x]
        code.append(number)

    return concat(map(str, code))


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

def move(location, direction, board):
    offset = direction_offset(direction)
    new_location = tuple_add(location, offset)

    if not is_valid_location(new_location, board):
        return location

    return new_location

def is_valid_location(location, board):
    (x, y) = location
    size = len(board)

    is_in_board = (
        x >= 0 and y >= 0 and
        x < size and y < size
    )

    if not is_in_board:
        return False

    return board[y][x] is not None
