from utils import *

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
    return [map(parse_instruction, line) for line in lines]


class Direction(Enum):
    Up = 1
    Right = 2
    Down = 3
    Left = 4

def move(location, direction, board):
    offset = {
        Direction.Up:    ( 0, -1),
        Direction.Down:  ( 0,  1),
        Direction.Left:  (-1,  0),
        Direction.Right: ( 1,  0),
    }[direction]

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
