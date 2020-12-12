from utils import *

FLOOR='.'
EMPTY='L'
OCCUPIED='#'

@check(2277)
def part1(state):
    ysize = len(state)
    xsize = len(state[0])

    def is_on_board(position):
        x, y = position
        return x >= 0 and y >= 0 and x < xsize and y < ysize

    def new_cell_state(position, last_state):
        x, y = position
        state = last_state[y][x]

        # Floor... floor never changes
        if state == FLOOR:
            return FLOOR

        neighbors = [n for n in neighbors8((x, y)) if is_on_board(n)]
        neighbor_states = [last_state[y][x] for (x, y) in neighbors]

        if state == EMPTY and not any(s == OCCUPIED for s in neighbor_states):
            return OCCUPIED

        if state == OCCUPIED and len([s for s in neighbor_states if s == OCCUPIED]) >= 4:
            return EMPTY

        return state

    last_state = None

    while state != last_state:
        last_state = state
        state = [[new_cell_state((x, y), state) for x in range(xsize)] for y in range(ysize)]

    return "".join(flatten(state)).count(OCCUPIED)

@check(2066)
def part2(state):
    ysize = len(state)
    xsize = len(state[0])
    directions = neighbors8((0, 0))

    def is_on_board(position):
        x, y = position
        return x >= 0 and y >= 0 and x < xsize and y < ysize

    def cell_in_direction(start, direction, state):
        cell = start

        while True:
            cell = tuple_add(cell, direction)
            if not is_on_board(cell):
                return None

            x, y = cell
            cell_state = state[y][x]
            if cell_state != FLOOR:
                return cell_state

    def new_cell_state(position, last_state):
        x, y = position
        state = last_state[y][x]

        # Floor... floor never changes
        if state == FLOOR:
            return FLOOR

        neighbors = [cell_in_direction(position, direction, last_state) for direction in directions]
        occupied_neighbors = [n for n in neighbors if n == OCCUPIED]

        if state == EMPTY and not occupied_neighbors:
            return OCCUPIED

        if state == OCCUPIED and len(occupied_neighbors) >= 5:
            return EMPTY

        return state

    last_state = None

    while state != last_state:
        last_state = state
        state = [[new_cell_state((x, y), state) for x in range(xsize)] for y in range(ysize)]

    return "".join(flatten(state)).count(OCCUPIED)

def transform_input(i):
    return [[seat for seat in line] for line in i.splitlines()]
