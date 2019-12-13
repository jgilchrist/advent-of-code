from utils import *

from .intcode import IntcodeVm

class Direction(Enum):
    Up = 1
    Right = 2
    Down = 3
    Left = 4

def move(location, direction):
    offset = {
        Direction.Up:    ( 0, -1),
        Direction.Down:  ( 0,  1),
        Direction.Left:  (-1,  0),
        Direction.Right: ( 1,  0),
    }[direction]

    new_location = tuple_add(location, offset)
    return new_location

def turn_clockwise(direction):
    return {
        Direction.Up:    Direction.Right,
        Direction.Down:  Direction.Left,
        Direction.Left:  Direction.Up,
        Direction.Right: Direction.Down,
    }[direction]

def turn_anticlockwise(direction):
    return {
        Direction.Up:    Direction.Left,
        Direction.Down:  Direction.Right,
        Direction.Left:  Direction.Down,
        Direction.Right: Direction.Up,
    }[direction]

def run_printer(program, add_initial_square):
    filled_squares = set()
    modified_squares = set()
    current_position = (0, 0)
    current_direction = Direction.Up

    if add_initial_square:
        filled_squares.add(current_position)

    vm = IntcodeVm(program)

    while not vm.halted:
        current_square = 1 if current_position in filled_squares else 0
        vm.add_input(current_square)

        next_color = vm.run_until_next_output()
        next_turn = vm.run_until_next_output()

        if next_color is None: break
        if next_turn is None: break

        if next_color:
            filled_squares.add(current_position)
        else:
            try:
                filled_squares.remove(current_position)
            except KeyError:
                pass

        modified_squares.add(current_position)

        if next_turn:
            current_direction = turn_clockwise(current_direction)
        else:
            current_direction = turn_anticlockwise(current_direction)

        current_position = move(current_position, current_direction)

    return filled_squares, modified_squares

def part1(program):
    filled_squares, modified_squares = run_printer(program, False)
    return len(modified_squares)

def part2(program):
    filled_squares, modified_squares = run_printer(program, True)

    xs = [x for x,y in filled_squares]
    ys = [y for x,y in filled_squares]
    minx, maxx = min(xs), max(xs)
    miny, maxy = min(ys), max(ys)

    lines = []
    for y in range(miny, maxy+1):
        line = ""
        for x in range(minx, maxx+1):
            line += "#" if (x,y) in filled_squares else "."
        lines.append(line)

    print()
    return "\n".join(lines)

def transform_input(i):
    return list(map(int, i.split(',')))
