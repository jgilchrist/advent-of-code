from utils import *

def assign_point(points, point):
    distances = { i: manhattan_distance(point, i) for i in points }

    min_key, min_value = argmin(distances)

    if sum(1 for k,v in distances.items() if v == min_value) != 1:
        return None

    return min_key

def is_point_in_region(points, point):
    sum_of_distances = sum(manhattan_distance(point, i) for i in points)
    return sum_of_distances < 10000

def part1(points):
    max_x = max(x for x,y in points)
    max_y = max(y for x,y in points)
    min_x = min(x for x,y in points)
    min_y = min(y for x,y in points)

    assignments = defaultdict(int)

    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            point = (x, y)
            assignment = assign_point(points, point)
            if assignment is not None:
                assignments[assignment] += 1

    max_key, max_value = argmax(assignments)
    return max_value

def part2(points):
    max_x = max(x for x,y in points)
    max_y = max(y for x,y in points)
    min_x = min(x for x,y in points)
    min_y = min(y for x,y in points)

    points_in_region = 0

    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            point = (x, y)
            if is_point_in_region(points, point):
                points_in_region += 1

    return points_in_region

def transform_input(i):
    points = i.splitlines()
    points = [point.split(',') for point in points]
    points = [(int(x), int(y)) for x,y in points]
    return points
