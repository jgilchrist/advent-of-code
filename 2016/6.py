from collections import Counter
from utils import transpose, lmap, concat

def part1(columns):
    most_frequent_letters = lmap(lambda c: get_nth_most_frequent(c, 0), columns)
    print(concat(most_frequent_letters))

def part2(columns):
    least_frequent_letters = lmap(lambda c: get_nth_most_frequent(c, -1), columns)
    print(concat(least_frequent_letters))

def get_nth_most_frequent(iterable, n):
    # Last [0] gets the value from the (value, occurences) pair
    return Counter(iterable).most_common()[n][0]

def transform_input(challenge_input):
    lines = challenge_input.splitlines()
    columns = transpose(lines)
    return columns
