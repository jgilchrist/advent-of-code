from utils import *

from enum import IntEnum

class Direction(IntEnum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

def go_in_direction(point, direction, amount):
    def get_offset_for_direction(direction):
        if direction == Direction.NORTH:
            return ( 0, -1)
        elif direction == Direction.SOUTH:
            return ( 0,  1)
        elif direction == Direction.WEST:
            return (-1,  0)
        elif direction == Direction.EAST:
            return ( 1,  0)

    offset = get_offset_for_direction(direction)
    return tuple_add(point, tuple_mul(offset, amount))

def rotate_direction(direction, amount):
    amount = amount // 90
    return (direction + amount) % 4

def rotate_point(point, amount):
    radians = -math.radians(amount)
    cos = math.cos(radians)
    sin = math.sin(radians)

    x, y = point
    newx = round(cos * x - sin * y)
    newy = round(sin * x + cos * y)
    return newx, newy

def part1(i):
    def go(state, instruction):
        command, amount = instruction
        position, direction = state

        if command == 'N':
            position = go_in_direction(position, Direction.NORTH, amount)
        elif command == 'S':
            position = go_in_direction(position, Direction.SOUTH, amount)
        elif command == 'E':
            position = go_in_direction(position, Direction.EAST, amount)
        elif command == 'W':
            position = go_in_direction(position, Direction.WEST, amount)
        elif command == 'F':
            position = go_in_direction(position, direction, amount)
        elif command == 'L':
            direction = rotate_direction(direction, -amount)
        elif command == 'R':
            direction = rotate_direction(direction, amount)

        return (position, direction)

    state = ((0, 0), Direction.EAST)

    for instruction in i:
        state = go(state, instruction)

    final_position, final_direction = state
    return manhattan_distance((0, 0), final_position)

def part2(i):
    def go(state, instruction):
        command, amount = instruction
        position, waypoint = state

        if command == 'N':
            waypoint = go_in_direction(waypoint, Direction.NORTH, amount)
        elif command == 'S':
            waypoint = go_in_direction(waypoint, Direction.SOUTH, amount)
        elif command == 'E':
            waypoint = go_in_direction(waypoint, Direction.EAST, amount)
        elif command == 'W':
            waypoint = go_in_direction(waypoint, Direction.WEST, amount)
        elif command == 'F':
            position = tuple_add(position, tuple_mul(waypoint, amount))
        elif command == 'L':
            waypoint = rotate_point(waypoint, amount)
        elif command == 'R':
            waypoint = rotate_point(waypoint, -amount)

        return (position, waypoint)

    state = ((0, 0), (10, -1))

    for instruction in i:
        state = go(state, instruction)

    final_position, final_waypoint = state
    return manhattan_distance((0, 0), final_position)

def transform_input(i):
    return [(line[0], int(line[1:])) for line in i.splitlines()]
