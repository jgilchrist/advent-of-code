from utils import *

LOWER_HALF = 0
UPPER_HALF = 1

ROW_NUMBERS = list(range(128))
COLUMN_NUMBERS = list(range(8))

def bsp_find(seats, directions):
    if len(directions) == 0:
        assert len(seats) == 1
        return seats[0]

    current_direction = directions[0]
    remaining_directions = directions[1:]

    half_way = len(seats) // 2

    remaining_seats = seats[:half_way] if current_direction == LOWER_HALF else seats[half_way:]

    return bsp_find(remaining_seats, remaining_directions)

def find_seat(directions):
    row_directions = directions[:-3]
    column_directions = directions[-3:]

    row_seat = bsp_find(ROW_NUMBERS, row_directions)
    column_seat = bsp_find(COLUMN_NUMBERS, column_directions)

    return (row_seat, column_seat)

def get_seat_id(seat):
    row, column = seat
    return row * 8 + column

@check(963)
def part1(i):
    return max(get_seat_id(find_seat(directions)) for directions in i)

@check(592)
def part2(i):
    occupied_seats = set(find_seat(directions) for directions in i)
    all_seats = set(itertools.product(ROW_NUMBERS, COLUMN_NUMBERS))

    unoccupied_seats = all_seats - occupied_seats

    unoccupied_seats_with_neighbors = [
        seat for seat in unoccupied_seats
        if all(neighbor in occupied_seats or neighbor not in all_seats for neighbor in neighbors4(seat))
    ]

    assert len(unoccupied_seats_with_neighbors) == 1
    return get_seat_id(unoccupied_seats_with_neighbors[0])

def transform_input(i):
    def transform_line(line):
        def is_lower_half(char): return char in ('F', 'L')
        return [LOWER_HALF if is_lower_half(char) else UPPER_HALF for char in line]

    return [transform_line(line) for line in i.splitlines()]
