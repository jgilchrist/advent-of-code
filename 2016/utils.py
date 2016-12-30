import itertools
import hashlib
import operator
from functools import reduce
from heapq import heappush, heappop

def lmap(fn, functor):
    return list(map(fn, functor))

def lfilter(fn, functor):
    return list(filter(fn, functor))

def tuple_add(a, b):
    return tuple(map(operator.add, a, b))

def tuple_mul(tup, n):
    return tuple(i * n for i in tup)

def pair_is_inverse(p1, p2):
    (p2x, p2y) = p2
    return p1 == (p2y, p2x)

def product(values):
    return reduce(operator.mul, values, 1)

def direction_offset(d):
    """This method is a reusable map from directional enums to offsets.
    The only caveat is that it requires enums to be IntEnum for int
    comparisons, and that the fields of the enum must be defined in
    compass order."""

    return {
        # Up
        1: ( 0, -1),
        # Down
        3: ( 0,  1),
        # Left
        4: (-1,  0),
        # Right
        2: ( 1,  0),
    }[d]

def transpose(list_of_lists):
    return map(list, zip(*list_of_lists))

def flatten(list_of_lists):
    return list(itertools.chain.from_iterable(list_of_lists))

def chunk_list(l, chunk_size):
    for i in range(0, len(l), chunk_size):
        yield l[i:i+chunk_size]

def zip_with_constant(const, iterable):
    """This is a trick to allow zipping an iterable against a single value"""
    return itertools.zip_longest([], iterable, fillvalue=const)

def concat(iterable):
    return "".join(iterable)

def astar_search(initial_state, heuristic_fn, generator_fn):

    def Path(previous, s):
        "A list of states which lead to state s"
        return ([] if (s is None) else Path(previous, previous[s]) + [s])

    next_states = [(heuristic_fn(initial_state), initial_state)]
    previous    = { initial_state: None }
    path_cost   = { initial_state: 0 }

    while next_states:
        (f, state) = heappop(next_states)

        if heuristic_fn(state) == 0:
            return Path(previous, state)

        for next_state in generator_fn(state):
            new_cost = path_cost[state] + 1

            if next_state not in path_cost or new_cost < path_cost[next_state]:
                heappush(next_states, (new_cost + heuristic_fn(next_state), next_state))
                path_cost[next_state] = new_cost
                previous[next_state] = state

def hash_utf(string):
    m = hashlib.md5()
    m.update(string.encode('utf-8'))
    return m.hexdigest()

def import_challenge(challenge_number):
    return __import__(str(challenge_number).zfill(2))

def read_input(challenge_number):
    challenge = import_challenge(challenge_number)

    filename = "inputs/{0:02d}.input".format(challenge_number)

    with open(filename) as f:
        challenge_input = f.read().strip()

    if hasattr(challenge, 'transform_input'):
        challenge_input = challenge.transform_input(challenge_input)

    return challenge_input
