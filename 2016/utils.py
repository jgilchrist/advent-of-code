def tuple_add(a, b):
    import operator
    return tuple(map(operator.add, a, b))

def tuple_mul(tup, n):
    return tuple(i * n for i in tup)
