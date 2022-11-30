from utils import *

@check(1050)
def part1(triangles):
    valid_triangles = filter(is_valid_triangle, triangles)
    return len(list(valid_triangles))

@check(1921)
def part2(untransposed_triangles):
    triangles = transpose(untransposed_triangles)
    triangles = flatten(triangles)
    triangles = chunk_list(triangles, 3)
    valid_triangles = filter(is_valid_triangle, triangles)
    return len(list(valid_triangles))


def is_valid_triangle(triangle_sides):
    (smallest, middle, largest) = sorted(triangle_sides)
    return smallest + middle > largest

def transform_input(challenge_input):
    triangles = challenge_input.splitlines()
    triangles = [t.split() for t in triangles]
    triangles = [map(int, t) for t in triangles]
    return triangles
