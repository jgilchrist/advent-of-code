def lmap(functor, fn):
    return list(map(functor, fn))

def tuple_add(a, b):
    import operator
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
