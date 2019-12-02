from utils import *

import re

def regex_replace_in_all(iterable, regex, replacement):
    return [re.sub(regex, replacement, line) for line in iterable]

def part1(i):
    strings = list(i)
    chars_in_file = sum(len(s) for s in strings)

    strings = ["".join(rest) for first_char, *rest, last_char in strings]
    strings = regex_replace_in_all(strings, r'\\\\', r'.')
    strings = regex_replace_in_all(strings, r'\\\"', r'.')
    strings = regex_replace_in_all(strings, r'\\x[0-9A-Fa-f]{2}', r'.')

    chars_in_memory = sum(len(s) for s in strings)

    return chars_in_file - chars_in_memory

def part2(i):
    strings = list(i)
    chars_in_memory = sum(len(s) for s in strings)

    strings = regex_replace_in_all(strings, '"', r'..')
    strings = regex_replace_in_all(strings, r'\\', r'..')
    strings = ['"' + s + '"' for s in strings]

    chars_in_file = sum(len(s) for s in strings)

    return chars_in_file - chars_in_memory

def transform_input(i):
    return i.splitlines()
