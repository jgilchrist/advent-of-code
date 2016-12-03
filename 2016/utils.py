import itertools
import operator

def lmap(fn, functor):
    return list(map(fn, functor))

def lfilter(fn, functor):
    return list(filter(fn, functor))

def tuple_add(a, b):
    return tuple(map(operator.add, a, b))

def tuple_mul(tup, n):
    return tuple(i * n for i in tup)

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

def merge_lists(list_of_lists):
    return list(itertools.chain.from_iterable(list_of_lists))

def chunk_list(l, chunk_size):
    for i in range(0, len(l), chunk_size):
        yield l[i:i+chunk_size]
