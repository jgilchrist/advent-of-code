from utils import *

def can_be_made_by(candidates, n):
    diffs = set()

    for i in candidates:
        if i in diffs:
            return True

        diff = n - i
        diffs.add(diff)

    return False

@check(26134589)
def part1(numbers):
    preamble_size = 25

    for number_with_prev in window(numbers, preamble_size + 1):
        prev = number_with_prev[:-1]
        n = number_with_prev[-1]

        if not can_be_made_by(prev, n):
            return n

@check(3535124)
def part2(numbers):
    n = part1(numbers)

    for window_size in range(2, len(numbers)):
        for win in window(numbers, window_size):
            if sum(win) == n:
                return min(win) + max(win)

def transform_input(i):
    return [int(line) for line in i.splitlines()]
