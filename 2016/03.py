from utils import *

def part1(triangles):
    valid_triangles = lfilter(is_valid_triangle, triangles)
    return len(valid_triangles)

def part2(untransposed_triangles):
    triangles = transpose(untransposed_triangles)
    triangles = flatten(triangles)
    triangles = chunk_list(triangles, 3)
    valid_triangles = lfilter(is_valid_triangle, triangles)
    return len(valid_triangles)


def is_valid_triangle(triangle_sides):
    (smallest, middle, largest) = sorted(triangle_sides)
    return smallest + middle > largest

def transform_input(challenge_input):
    triangles = challenge_input.splitlines()
    triangles = [t.split() for t in triangles]
    triangles = [lmap(int, t) for t in triangles]
    return triangles
