from utils import *

class Move(Enum):
    West = 0,
    NorthWest = 1,
    NorthEast = 2,
    East = 3,
    SouthEast = 4,
    SouthWest = 5,

def neighbors_hex(coord):
    return [
        make_move(coord, Move.West),
        make_move(coord, Move.NorthWest),
        make_move(coord, Move.NorthEast),
        make_move(coord, Move.East),
        make_move(coord, Move.SouthEast),
        make_move(coord, Move.SouthWest),
    ]

def make_move(coord, move):
    x, y, z = coord

    if move == Move.West:
        return (x - 1, y + 1, z)
    elif move == Move.NorthWest:
        return (x - 1, y, z + 1)
    elif move == Move.NorthEast:
        return (x, y - 1, z + 1)
    elif move == Move.East:
        return (x + 1, y - 1, z)
    elif move == Move.SouthEast:
        return (x + 1, y, z - 1)
    elif move == Move.SouthWest:
        return (x, y + 1, z - 1)

def get_coord(moves):
    current = (0, 0, 0)
    for move in moves:
        current = make_move(current, move)

    return current

def is_tile_flipped(tile, neighbors):
    if tile:
        return len(neighbors) == 1 or len(neighbors) == 2
    else:
        return len(neighbors) == 2

def step(tiles):
    new_tiles = set()
    tiles_that_can_flip = set(flatten([[tile, *neighbors_hex(tile)] for tile in tiles]))

    for tile in tiles_that_can_flip:
        tile_neighbors = set(neighbors_hex(tile))
        flipped_neighbors = set.intersection(tiles, tile_neighbors)
        is_flipped = tile in tiles

        if is_tile_flipped(is_flipped, flipped_neighbors):
            new_tiles.add(tile)

    return new_tiles

def get_flipped_tiles_after_moveset(moveset):
    flips = [get_coord(moves) for moves in moveset]
    flip_counts = Counter(flips)
    return set([tile for tile, flip_count in flip_counts.items() if flip_count % 2 != 0])

@check(523)
def part1(moveset):
    tiles = get_flipped_tiles_after_moveset(moveset)
    return len(tiles)

@check(4225)
def part2(moveset):
    tiles = get_flipped_tiles_after_moveset(moveset)

    for turn in range(100):
        tiles = step(tiles)

    return len(tiles)

def transform_input(i):
    def parse_line(line):
        idx = 0
        moves = []

        while idx < len(line):
            char1 = line[idx]
            idx += 1

            if char1 == "w":
                moves.append(Move.West)
            elif char1 == "e":
                moves.append(Move.East)
            elif char1 == "n":
                char2 = line[idx]
                idx += 1

                if char2 == "w":
                    moves.append(Move.NorthWest)
                elif char2 == "e":
                    moves.append(Move.NorthEast)
                else:
                    assert False

            elif char1 == "s":
                char2 = line[idx]
                idx += 1

                if char2 == "w":
                    moves.append(Move.SouthWest)
                elif char2 == "e":
                    moves.append(Move.SouthEast)
                else:
                    assert False

            else:
                assert False

        return moves

    return [parse_line(line) for line in i.splitlines()]
