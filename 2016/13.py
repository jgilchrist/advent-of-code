from utils import *

def heuristic(state):
    # The goal is at (31, 39)
    (x, y) = state
    return abs(31 - x) + abs(39 - y)

@check(92)
def part1(seed):
    path = astar_search((1, 1), heuristic, partial(adjacent_spaces, seed=seed))
    return len(path) - 1

@check(124)
def part2(seed):
    initial_state = (1, 1)
    next_states = deque([initial_state])
    distance_from_start = { initial_state: 0 }

    while next_states:
        state = next_states.popleft()
        if distance_from_start[state] < 50:
            for new_state in adjacent_spaces(state, seed):
                if new_state not in distance_from_start:
                    next_states.append(new_state)
                    distance_from_start[new_state] = distance_from_start[state] + 1

    return len(distance_from_start)


def adjacent_spaces(position, seed):
    return [s for s in neighbors4(position) if is_space(s, seed)]

def is_space(position, seed):
    x, y = position
    computed_sum = x*x + 3*x + 2*x*y + y + y*y + seed

    number_of_ones = bin(computed_sum).count('1')
    return number_of_ones % 2 == 0 and x >= 0 and y >= 0

def transform_input(challenge_input):
    return int(challenge_input)
