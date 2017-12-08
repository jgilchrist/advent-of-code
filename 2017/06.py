from utils import *

def part1(i):
    return run_on_repeated_state(i, lambda seen, state: len(seen))

def part2(i):
    return run_on_repeated_state(i, lambda seen, state: len(seen) - seen.index(state))

def run_on_repeated_state(state, fn_when_seen):
    seen = [state]

    while True:
        state = generate_new_state(state)
        if state in seen:
            return fn_when_seen(seen, state)
        seen.append(state)

def generate_new_state(state):
    max_index, max_value = max(enumerate(state), key=operator.itemgetter(1))

    new_state = list(state)
    new_state[max_index] = 0
    for i in range(1, max_value + 1):
        new_state[(max_index + i) % len(state)] += 1

    return new_state

def transform_input(i):
    return lmap(int, i.split('\t'))
