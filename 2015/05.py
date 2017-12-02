from utils import *

import re

def part1(i):
    def is_nice(line):
        count_occurences = lambda letter: 1 if letter in line else 0

        number_of_vowels = (
            line.count('a') +
            line.count('e') +
            line.count('i') +
            line.count('o') +
            line.count('u')
        )

        doesnt_match_disallowed_strings = ('ab' not in line and 'cd' not in line and 'pq' not in line and 'xy' not in line)

        double_letter = re.search(r'(.)\1', line) is not None

        return number_of_vowels >= 3 and doesnt_match_disallowed_strings and double_letter

    return sum(1 for line in i if is_nice(line))

def part2(i):
    def is_nice(line):
        has_matching_pair = re.search(r'(..).*\1', line)
        has_xyx = re.search(r'(.)[^\1]\1', line)

        return has_matching_pair and has_xyx

    return sum(1 for line in i if is_nice(line))

def transform_input(i):
    return i.splitlines()
