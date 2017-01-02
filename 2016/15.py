from utils import *

def part1(challenge_info):
    npositions, initial_state = challenge_info
    return run_machine(npositions, initial_state)

def part2(challenge_info):
    npositions, initial_state = challenge_info

    # Extend the initial position with a new disk
    npositions = (*npositions, 11)
    initial_state = (*initial_state, 0)

    return run_machine(npositions, initial_state)


def run_machine(npositions, initial_state):
    states = generate_states(npositions, initial_state)

    for index, state in enumerate(states):
        if all(pos == 0 for pos in state):
            return index

def generate_states(npositions, initial_state):
    "An infinite generator of next states based on the challenge's rules for state generation"

    number_of_disks = len(npositions)

    tuple_of_count_generators = map(count, initial_state)
    states = zip(*tuple_of_count_generators)

    # Apply an addition mask to the state (1, 2, 3, 4, ...) to account for time passing
    time_mask = tuple(islice(count(1), number_of_disks))
    time_adjusted_states = (tuple_add(s, time_mask) for s in states)

    def mask(state, modulo_mask):
        return tuple(i % mask for (i, mask) in zip(state, modulo_mask))

    return (mask(s, npositions) for s in time_adjusted_states)

def transform_input(challenge_input):
    disk_lines = challenge_input.splitlines()

    npositions_regex = re.compile(r'(\d+) positions')
    initial_position_regex = re.compile(r'at position (\d+)')

    npositions = tuple(map(int, (npositions_regex.findall(line)[0] for line in disk_lines)))
    initial_positions = tuple(map(int, (initial_position_regex.findall(line)[0] for line in disk_lines)))

    return (npositions, initial_positions)
