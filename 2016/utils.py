import itertools
import operator
from functools import reduce

def lmap(fn, functor):
    return list(map(fn, functor))

def lfilter(fn, functor):
    return list(filter(fn, functor))

def tuple_add(a, b):
    return tuple(map(operator.add, a, b))

def tuple_mul(tup, n):
    return tuple(i * n for i in tup)

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
    return lmap(list, zip(*list_of_lists))

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


def read_input(challenge_number):
    challenge = __import__(str(challenge_number))

    filename = "{}.input".format(challenge_number)

    with open(filename) as f:
        challenge_input = f.read().strip()

    if hasattr(challenge, 'transform_input'):
        challenge_input = challenge.transform_input(challenge_input)

    return challenge_input
