from collections import namedtuple
from functools import partial
from utils import astar_search, hash_utf, tuple_add

State = namedtuple('State', ['position', 'path'])

deltas = [
    # In the form: (md5 character index, direction, delta)
    (0, 'U', ( 0, -1)),
    (1, 'D', ( 0, +1)),
    (2, 'L', (-1,  0)),
    (3, 'R', (+1,  0)),
]

def heuristic(state):
    position, path = state
    x, y = position
    return abs(3 - x) + abs(3 - y)

def part1(passcode):
    path = astar_search(State((0, 0), ''), heuristic, partial(get_next_states, seed=passcode))
    return path[-1].path

def part2(passcode):
    longest_path = 0
    next_states = [State((0, 0), '')]

    while next_states:
        state = next_states.pop()

        if state.position == (3, 3):
            longest_path = max(longest_path, len(state.path))
        else:
            next_states += get_next_states(state, passcode)

    return longest_path

def get_next_states(state, seed):
    position, path = state
    state_hash = hash_utf(seed + path)

    next_states = []

    for (md5_index, direction, delta) in deltas:
        new_position = tuple_add(position, delta)
        new_path = path + direction

        if can_traverse(state_hash[md5_index]) and is_valid_position(new_position):
            next_states.append(State(new_position, new_path))

    return next_states

def can_traverse(c):
    return c in 'bcdef'

def is_valid_position(position):
    x, y = position
    return x >= 0 and y >= 0 and x < 4 and y < 4
