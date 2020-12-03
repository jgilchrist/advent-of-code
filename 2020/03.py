from utils import *

def count_trees_for_slope(tree_map, slope):
    x, y = 0, 0
    vx, vy = slope
    width, height = len(tree_map[0]), len(tree_map)
    trees = 0

    while y < height:
        trees += 1 if tree_map[y][x] == "#" else 0
        x, y = (x + vx) % width, y + vy

    return trees

@check(151)
def part1(i):
    return count_trees_for_slope(i, (3, 1))

@check(7540141059)
def part2(i):
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    trees = [count_trees_for_slope(i, slope) for slope in slopes]
    return math.prod(trees)

def transform_input(i):
    return i.splitlines()
