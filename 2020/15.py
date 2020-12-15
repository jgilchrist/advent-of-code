from utils import *

def run_until(i, until):
    spoken_numbers = { n: idx + 1 for (idx, n) in enumerate(i[:-1]) }
    last_number = i[-1]

    for tick in range(len(i), until):
        next_number = 0 if last_number not in spoken_numbers else tick - spoken_numbers[last_number]
        spoken_numbers[last_number] = tick
        last_number = next_number

    return next_number

@check(273)
def part1(i):
    return run_until(i, 2020)

@check(47205)
def part2(i):
    return run_until(i, 30000000)

def transform_input(i):
    return list(get_all_positive_numbers(i))
