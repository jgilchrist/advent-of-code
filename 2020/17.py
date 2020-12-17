from utils import *

def neighbors(point):
    dimensions = len(point)
    diffs = itertools.product((-1, 0, 1), repeat=dimensions)

    for diff in diffs:
        if any(diff):
            yield tuple_add(point, diff)

def get_bounding_box(cubes):
    def get_bounding_box_for_dimension(cubes, i):
        coords = [c[i] for c in cubes]
        return min(coords), max(coords)

    dimensions = len(next(iter(cubes)))
    return tuple(get_bounding_box_for_dimension(cubes, d) for d in range(dimensions))

def all_points(bounding_box):
    possible_coords = [extended_range(dimension_range) for dimension_range in bounding_box]
    return itertools.product(*possible_coords)

def extended_range(tup):
    tup_min, tup_max = tup
    return inclusive_range((tup_min - 1, tup_max + 1))

def inclusive_range(tup):
    tup_min, tup_max = tup
    return range(tup_min, tup_max + 1)

def is_cube_active(cube, neighbors):
    return len(neighbors) in (2, 3) if cube else len(neighbors) == 3

def step(cubes):
    new_cubes = set()
    bounding_box = get_bounding_box(cubes)

    for point in all_points(bounding_box):
        cube_neighbors = set(neighbors(point))
        active_neighbors = set.intersection(cubes, cube_neighbors)
        is_active = point in cubes

        if is_cube_active(is_active, active_neighbors):
            new_cubes.add(point)

    return new_cubes

def draw3d(cubes):
    xbounds, ybounds, zbounds = get_bounding_box(cubes)

    print()
    for z in inclusive_range(zbounds):
        print(f"z={z}")
        for y in inclusive_range(ybounds):
            for x in inclusive_range(xbounds):
                print("#" if (x, y, z) in cubes else ".", end="")
            print()
        print()
    print()

@check(348)
def part1(cubes):
    # Embed cubes in 3 dimensions
    cubes = set((x, y, 0) for x, y in cubes)

    for i in range(6):
        cubes = step(cubes)

    return len(cubes)

@check(2236)
def part2(cubes):
    # Embed cubes in 4 dimensions
    cubes = set((x, y, 0, 0) for x, y in cubes)

    for i in range(6):
        cubes = step(cubes)

    return len(cubes)

def transform_input(i):
    lines = i.splitlines()

    cubes = set()

    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char == '#':
                cubes.add((x, y))

    return cubes
