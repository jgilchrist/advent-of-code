from utils import *

def part1(ranges):
    return next(all_unblocked_numbers(ranges))

def part2(ranges):
    return len(list(all_unblocked_numbers(ranges)))

def all_unblocked_numbers(ranges):
    i = 0

    # Step through all low/high pairs
    # For each, iterate through all unseen numbers from the last high
    # to the current low, then update 'i' to account for the numbers
    # we've seen and the current range.
    for (low, high) in ranges:
        yield from range(i, low)
        i = max(i, high + 1)

def transform_input(challenge_input):
    lines = challenge_input.splitlines()
    return sorted(tuple(map(int, l.split("-"))) for l in lines)
