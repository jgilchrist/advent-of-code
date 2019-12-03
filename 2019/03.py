from utils import *

from enum import Enum

class Direction(Enum):
    UP = 0
    DOWN = 1
    LEFT = 2
    RIGHT = 3

def get_offset_for_direction(direction):
    if direction == Direction.UP:
        return ( 0, -1)
    elif direction == Direction.DOWN:
        return ( 0,  1)
    elif direction == Direction.LEFT:
        return (-1,  0)
    elif direction == Direction.RIGHT:
        return ( 1,  0)
    else:
        raise Exception("Invalid direction")

def go_in_direction(point, direction):
    offset = get_offset_for_direction(direction)
    return tuple_add(point, offset)

def get_all_points_and_distances_for_line(instructions):
    loc = (0, 0)
    points = set()
    distances_to_points = {}
    number_of_steps = 0

    for (direction, number) in instructions:
        for n in range(number):
            number_of_steps += 1
            loc = go_in_direction(loc, direction)
            points.add(loc)

            if loc not in distances_to_points:
                distances_to_points[loc] = number_of_steps

    return points, distances_to_points

def part1(i):
    points_and_distances = [get_all_points_and_distances_for_line(line) for line in i]
    all_points = [points for (points, distances) in points_and_distances]
    intersections = set.intersection(*all_points)
    distance_from_centre = {point: manhattan_distance(point, (0, 0)) for point in intersections}
    (key, value) = argmin(distance_from_centre)
    return value

def part2(i):
    points_and_distances = [get_all_points_and_distances_for_line(line) for line in i]
    all_points = [points for (points, distances) in points_and_distances]
    all_distances = [distances for (points, distances) in points_and_distances]
    intersections = set.intersection(*all_points)

    distance_to_each_intersection = {
        intersection: sum(distance[intersection] for distance in all_distances)
        for intersection in intersections
    }
    (key, value) = argmin(distance_to_each_intersection)
    return value

def transform_input(i):
    lines = i.splitlines()
    line_instructions = [line.split(',') for line in lines]

    fns = [
        ('U(\d+)', lambda p1: (Direction.UP, int(p1))),
        ('D(\d+)', lambda p1: (Direction.DOWN, int(p1))),
        ('L(\d+)', lambda p1: (Direction.LEFT, int(p1))),
        ('R(\d+)', lambda p1: (Direction.RIGHT, int(p1))),
    ]

    def function_for(instruction):
        for (regex, fn) in fns:
            if re.match(regex, instruction):
                groups = re.search(regex, instruction).groups()
                return fn(*groups)

    return [lmap(function_for, line) for line in line_instructions]
