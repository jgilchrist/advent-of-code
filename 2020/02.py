from utils import *

import itertools
import math

@check(638)
def part1(i):
    def is_valid_password(item):
        rangeStart, rangeEnd, char, password = item
        return rangeStart <= password.count(char) <= rangeEnd

    return len(list(filter(is_valid_password, i)))

@check(699)
def part2(i):
    def is_valid_password(item):
        idx1, idx2, char, password = item
        return (password[idx1 - 1] == char) != (password[idx2 - 1] == char)

    return len(list(filter(is_valid_password, i)))

def transform_input(i):
    def parse_password_info(line):
        p1, p2, p3 = line.split()
        n1, n2 = get_all_positive_numbers(p1)
        assert len(p2) == 2
        return (n1, n2, p2[0], p3)

    return [parse_password_info(line) for line in i.splitlines()]
