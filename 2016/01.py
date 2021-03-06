from utils import *

@check(252)
def part1(instructions):
    state = State((0, 0), Direction.North)

    for instruction in instructions:
        (state, _) = move(state, instruction)

    return distance_from_start(state.location)

@check(143)
def part2(instructions):
    state = State((0, 0), Direction.North)
    visited_locations = []

    for instruction in instructions:
        (state, visited_locations_while_moving) = move(state, instruction)

        for loc in visited_locations_while_moving:
            if loc in visited_locations:
                return distance_from_start(loc)

            visited_locations.append(loc)


def transform_input(challenge_input):
    def parse_instruction(instruction):
        direction = instruction[0]
        amount = int(instruction[1:])
        return Instruction(direction, amount)

    instructions = challenge_input.replace(' ', '').split(',')
    instructions = map(parse_instruction, instructions)
    return instructions


Instruction = namedtuple('Instruction', ['direction', 'amount'])
State = namedtuple('State', ['location', 'facing'])

class Direction(Enum):
    North = 1
    East = 2
    South = 3
    West = 4


def move(state, instruction):
    new_facing = turn(state.facing, instruction.direction)
    new_locations = move_in_direction(state.location, new_facing, instruction.amount)
    new_location = new_locations[-1]
    return (State(new_location, new_facing), new_locations)

def turn(facing, direction):
    def right(d):
        return {
            Direction.North: Direction.East,
            Direction.East: Direction.South,
            Direction.South: Direction.West,
            Direction.West: Direction.North,
        }[d]

    def left(d):
        return {
            Direction.North: Direction.West,
            Direction.East: Direction.North,
            Direction.South: Direction.East,
            Direction.West: Direction.South,
        }[d]

    if direction == 'L':
        return left(facing)
    else:
        return right(facing)

def move_in_direction(location, direction, amount):
    direction_adjustment = {
        Direction.North: ( 0, -1),
        Direction.South: ( 0,  1),
        Direction.West:  (-1,  0),
        Direction.East:  ( 1,  0),
    }[direction]

    return [tuple_add(location, tuple_mul(direction_adjustment, i)) for i in range(1, amount+1)]

def distance_from_start(location):
    x = abs(location[0])
    y = abs(location[1])
    return x + y
