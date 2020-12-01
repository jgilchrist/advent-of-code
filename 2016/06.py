from utils import *

@check("afwlyyyq")
def part1(columns):
    get_most_frequent = partial(get_nth_most_frequent, nth=0)
    most_frequent_letters = map(get_most_frequent, columns)
    return concat(most_frequent_letters)

@check("bhkzekao")
def part2(columns):
    get_least_frequent = partial(get_nth_most_frequent, nth=-1)
    least_frequent_letters = map(get_least_frequent, columns)
    return concat(least_frequent_letters)

def get_nth_most_frequent(iterable, nth):
    # Last [0] gets the value from the (value, occurences) pair
    return Counter(iterable).most_common()[nth][0]

def transform_input(challenge_input):
    lines = challenge_input.splitlines()
    columns = transpose(lines)
    return columns
