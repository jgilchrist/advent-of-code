from utils import *

@check(2035)
def part1(first_row):
    return count_safe_tiles_in_n_rows(first_row, 40)

@check(20000577)
@slow
def part2(first_row):
    return count_safe_tiles_in_n_rows(first_row, 400000)

def count_safe_tiles_in_n_rows(first_row, n):
    row = str(first_row)
    rows = [row]

    while len(rows) != n:
        row = next_row("." + row + ".")
        rows.append(row)

    return concat(rows).count(".")

def next_row(row):
    return concat([
        "^" if is_trap(left, right) else "."
        for (left, _, right) in window(row, 3)
    ])

def is_trap(left, right):
    "The trap rules can be reduced to left != right"
    return left != right

def window(iterable, size):
    "Returns a sliding window (of width n) over data from the iterable"

    it = iter(iterable)
    result = tuple(islice(it, size))
    if len(result) == size:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result
