from utils import *

from collections import Counter

def matches_password_criteria(password):
    password = str(password)
    is_increasing = sorted(password) == list(password)

    if not is_increasing:
        return False

    has_repetitions = max(Counter(password).values()) > 1
    return has_repetitions

def matches_part2_password_criteria(password):
    password = str(password)
    is_increasing = sorted(password) == list(password)

    if not is_increasing:
        return False

    has_one_twox_repetition = 2 in Counter(password).values()
    return has_one_twox_repetition

@check(921)
def part1(i):
    a, b = i
    potential_passwords = range(a, b+1)
    return len(list(filter(matches_password_criteria, potential_passwords)))

@check(603)
def part2(i):
    a, b = i
    potential_passwords = range(a, b+1)
    return len(list(filter(matches_part2_password_criteria, potential_passwords)))

def transform_input(i):
    a, b = i.split('-')
    return int(a), int(b)
