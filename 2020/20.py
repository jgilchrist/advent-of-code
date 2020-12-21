from utils import *

import random

visualise = True

def flip(contents):
    contents = list(reversed(contents))
    return contents

def generate_tile_orientations(contents):
    return [
        contents,
        rotate(contents),
        rotate(rotate(contents)),
        rotate(rotate(rotate(contents))),
        flip(contents),
        rotate(flip(contents)),
        rotate(rotate(flip(contents))),
        rotate(rotate(rotate(flip(contents)))),
    ]

def top_edge(tile):
    return "".join(tile[0])

def bottom_edge(tile):
    return "".join(tile[-1])

def left_edge(tile):
    return "".join(row[0] for row in tile)

def right_edge(tile):
    return "".join(row[-1] for row in tile)

def get_map(tiles):
    # Start by placing the first tile, and try and match others to it
    placed_tiles = { (0, 0): tiles[0] }
    unplaced_tiles = [(n, generate_tile_orientations(t)) for n, t in tiles[1:]]

    def placed_numbers(placed):
        placed = [n for n, _ in placed.values()]
        return placed

    new_placed_tiles = {}

    while unplaced_tiles:

        for placed_tile_coord, tile in placed_tiles.items():
            (x, y) = placed_tile_coord
            _, contents = tile

            for unplaced_tile in unplaced_tiles:
                n, orientations = unplaced_tile

                for orientation in orientations:
                    if top_edge(contents) == bottom_edge(orientation):
                        new_placed_tiles[(x, y-1)] = (n, orientation)
                    elif bottom_edge(contents) == top_edge(orientation):
                        new_placed_tiles[(x, y+1)] = (n, orientation)
                    elif left_edge(contents) == right_edge(orientation):
                        new_placed_tiles[(x-1, y)] = (n, orientation)
                    elif right_edge(contents) == left_edge(orientation):
                        new_placed_tiles[(x+1, y)] = (n, orientation)

            unplaced_tiles = [(n, orientations) for (n, orientations) in unplaced_tiles if n not in placed_numbers(placed_tiles)]

        placed_tiles.update(new_placed_tiles)
        new_placed_tiles = {}

        if visualise:
            draw_map(placed_tiles)
            draw_numbers(placed_tiles)

    return placed_tiles

def trim_tile(tile):
    n, contents = tile
    return n, [line[1:-1] for line in contents[1:-1]]

def render_whole_map(placed_tiles):
    whole_map = []

    random_tile = list(placed_tiles.values())[0]
    _, tile_contents = random_tile
    tile_size = len(tile_contents)

    coords = placed_tiles.keys()

    xs = [x for (x, y) in coords]
    ys = [y for (x, y) in coords]

    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)

    for tile_y in range(min_y, max_y+1):
        for tile_row_number in range(tile_size):
            row = []
            for tile_x in range(min_x, max_x+1):
                tile_at_xy = placed_tiles[(tile_x, tile_y)]
                _, tile_contents = tile_at_xy
                tile_row = tile_contents[tile_row_number]
                row.extend(list(tile_row))

            whole_map.append(row)

    return whole_map

sea_monster = """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """

sea_monster_l1, sea_monster_l2, sea_monster_l3 = sea_monster.splitlines()
sea_monster_parts = [sea_monster_l1, sea_monster_l2, sea_monster_l3]


def get_sea_monsters(whole_map):
    sea_monster_length = len(sea_monster_l1)

    sea_monsters = []

    def is_match(line, pattern):
        return all(pattern_char == line_char if pattern_char == "#" else True
            for line_char, pattern_char in zip(line, pattern)
        )

    for start_y, (l1, l2, l3) in enumerate(window(whole_map, 3)):
        for start_x, (l1_window, l2_window, l3_window) in enumerate(zip(window(l1, sea_monster_length), window(l2, sea_monster_length), window(l3, sea_monster_length))):
            is_sea_monster = is_match(l1_window, sea_monster_l1) and is_match(l2_window, sea_monster_l2) and is_match(l3_window, sea_monster_l3)

            if is_sea_monster:
                sea_monsters.append((start_x, start_y))

    return sea_monsters

def mark_monsters_on_map(whole_map, sea_monsters):
    def mark_char(line, line_number, monster):
        pass

    def mark_monster_in_line(line_number, line, monster):
        monster_x, monster_y = monster
        monster_lines = list(range(monster_y, monster_y + 3))

        if line_number not in monster_lines:
            return line

        matching_monster_line = sea_monster_parts[line_number - monster_y]

        padding_left = monster_x
        padding_right = len(line) - monster_x - len(matching_monster_line)

        padded_monster_line = " " * padding_left + matching_monster_line + " " * padding_right

        assert len(padded_monster_line) == len(line)

        new_line = [
            "O" if pattern_char == "#" else line_char
            for line_char, pattern_char in zip(line, padded_monster_line)
        ]

        return new_line

    def mark_monster_on_map(current_map, monster):
        lines = [
            mark_monster_in_line(line_number, line, monster)
            for line_number, line in enumerate(current_map)
        ]

        return lines

    for monster in sea_monsters:
        whole_map = mark_monster_on_map(whole_map, monster)

    return whole_map

@check(16937516456219)
def part1(tiles):
    placed_tiles = get_map(tiles)
    coords = placed_tiles.keys()

    xs = [x for (x, y) in coords]
    ys = [y for (x, y) in coords]

    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)

    corner_tiles = [
        placed_tiles[(min_x, min_y)],
        placed_tiles[(min_x, max_y)],
        placed_tiles[(max_x, min_y)],
        placed_tiles[(max_x, max_y)]
    ]

    corner_tile_numbers = [n for n, contents in corner_tiles]

    return math.prod(corner_tile_numbers)

@check(1858)
def part2(tiles):
    placed_tiles = get_map(tiles)

    trim_tiles = {
        coord: trim_tile(tile)
        for coord, tile in placed_tiles.items()
    }

    whole_map = render_whole_map(trim_tiles)

    map_orientations = generate_tile_orientations(whole_map)

    orientation_with_sea_monsters = [o for o in map_orientations if len(get_sea_monsters(o)) > 0][0]
    sea_monsters_in_map = get_sea_monsters(orientation_with_sea_monsters)
    map_with_monsters = mark_monsters_on_map(orientation_with_sea_monsters, sea_monsters_in_map)

    rendered_map_with_monsters = "\n".join("".join(line) for line in map_with_monsters)
    rendered_map_with_monsters = "".join([f'\033[93m{c}\033[0m' if c == "O" else c for c in rendered_map_with_monsters])

    if visualise:
        print()
        print(rendered_map_with_monsters)

    return rendered_map_with_monsters.count("#")

def transform_input(i):
    def parse_tile(t):
        header, *rest = t.splitlines()
        tile_number = list(get_all_positive_numbers(header))[0]
        return tile_number, rest

    tiles = i.split("\n\n")
    return [parse_tile(t) for t in tiles]

def draw_numbers(m):
    random_tile = list(m.values())[0]
    _, tile_contents = random_tile
    tile_size = len(tile_contents)

    blank_row = "    "

    coords = m.keys()

    xs = [x for (x, y) in coords]
    ys = [y for (x, y) in coords]

    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)

    print()
    for tile_y in range(min_y, max_y+1):
        for tile_x in range(min_x, max_x+1):
            try:
                tile_at_xy = m[(tile_x, tile_y)]
                n, _ = tile_at_xy
                print(n, end="")
            except KeyError:
                print(blank_row, end="")

            print(" ", end="")

        print()

def draw_map(m):
    random_tile = list(m.values())[0]
    _, tile_contents = random_tile
    tile_size = len(tile_contents)

    blank_row = " " * tile_size

    coords = m.keys()

    xs = [x for (x, y) in coords]
    ys = [y for (x, y) in coords]

    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)

    print()
    for tile_y in range(min_y, max_y+1):
        for tile_row_number in range(tile_size):
            for tile_x in range(min_x, max_x+1):
                try:
                    tile_at_xy = m[(tile_x, tile_y)]
                    _, tile_contents = tile_at_xy
                    tile_row = tile_contents[tile_row_number]
                    print("".join(tile_row), end="")
                except KeyError:
                    print(blank_row, end="")

                print(" ", end="")

            print()

        print()

def print_tile(t):
    number, contents = t

    for row in contents:
        print("".join(row))
