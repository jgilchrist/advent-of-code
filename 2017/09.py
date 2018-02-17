from utils import *

import re

def part1(i):
    i = '{{{},{},{{}}}}'
    i = re.sub('!.', '', i)
    i = re.sub('<[^>]*?>', '', i)
    i = re.sub(',', '', i)

    results = []
    while True:
        number_of_results = len(re.findall('{}', i))
        results.append(number_of_results)
        i = re.sub('{}', '', i)

        if not i: break

    multipliers = range(len(results), 0, -1)

    result = 0
    for (number_at_depth, multiplier) in zip(results, multipliers):
        result += number_at_depth * multiplier

    return result

def part2(i):
    pass

# def transform_input(i):
#     return i
