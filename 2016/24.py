from utils import *

State = namedtuple('State', ['position', 'numbers_seen'])


def part1(challenge_info):
    def heuristic(numbers_to_visit, state):
        return numbers_to_visit - len(state.numbers_seen)

    maze, numbers, initial_position = challenge_info
    initial_state = State(initial_position, '0')
    numbers_to_visit = len(numbers)

    path = astar_search(initial_state, partial(heuristic, numbers_to_visit), partial(next_states, maze, numbers))

    return len(path) - 1


def part2(challenge_info):
    def heuristic(start_position, numbers_to_visit, state):
        (startx, starty) = start_position
        (curx, cury) = state.position
        distance_from_start = abs(curx - startx) + abs(cury - starty)

        return distance_from_start + numbers_to_visit - len(state.numbers_seen)

    maze, numbers, initial_position = challenge_info
    initial_state = State(initial_position, '0')
    numbers_to_visit = len(numbers)

    path = astar_search(initial_state, partial(heuristic, initial_position, numbers_to_visit), partial(next_states, maze, numbers))

    return len(path) - 1


def next_states(maze, numbers, state):
    states = []
    for space in adjacent_spaces(maze, state.position):
        if space in numbers and not numbers[space] in state.numbers_seen:
            states.append(State(space, state.numbers_seen + numbers[space]))
        else:
            states.append(State(space, state.numbers_seen))
    return states

def adjacent_spaces(maze, position):
    return [s for s in neighbors4(position) if is_space(maze, s)]

def neighbors4(position):
    (x, y) = position
    return [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]

def is_space(maze, point):
    (x, y) = point
    return maze[y][x]

def transform_input(challenge_input):
    def parse_line(line):
        return [c != "#" for c in line]

    def get_numbers(lines):
        numbers = {}
        starting_position = None

        for y, line in enumerate(lines):
            for x, c in enumerate(line):
                if c in '0123456789':
                    numbers[(x, y)] = c

                if c == '0':
                    starting_position = (x, y)

        return numbers, starting_position


    lines = challenge_input.splitlines()

    maze = list(map(parse_line, lines))
    numbers, initial_position = get_numbers(lines)

    return maze, numbers, initial_position
