from utils import *

bin_to_dec = lambda b: int(b, 2)

def most_common_in(l):
    counter = Counter(l)
    most_common = counter.most_common()
    (el, count) = most_common[0]

    if len(most_common) > 1:
        (el2, count2) = most_common[1]
        return el if count != count2 else '1'

    return el

def least_common_in(l):
    counter = Counter(l)
    least_common = list(reversed(counter.most_common()))
    (el, count) = least_common[0]

    if len(least_common) > 1:
        (el2, count2) = least_common[1]
        return el if count != count2 else '0'

    return el

def part1(i):
    cols = list(transpose(i))
    gamma_rate = bin_to_dec(concat(lmap(most_common_in, cols)))
    epsilon_rate = bin_to_dec(concat(lmap(least_common_in, cols)))
    return gamma_rate * epsilon_rate

def part2(i):
    cols = list(transpose(i))

    oxygen_candidates = list(i)
    co2_candidates = list(i)

    current_oxygen_bit_idx = 0
    while len(oxygen_candidates) != 1:
        oxygen_candidate_cols = list(transpose(oxygen_candidates))
        most_common = most_common_in(oxygen_candidate_cols[current_oxygen_bit_idx])
        oxygen_candidates = list(filter(lambda c: c[current_oxygen_bit_idx] == most_common, oxygen_candidates))
        current_oxygen_bit_idx += 1

    current_co2_bit_idx = 0
    while len(co2_candidates) != 1:
        co2_candidate_cols = list(transpose(co2_candidates))
        least_common = least_common_in(co2_candidate_cols[current_co2_bit_idx])
        co2_candidates = list(filter(lambda c: c[current_co2_bit_idx] == least_common, co2_candidates))
        current_co2_bit_idx += 1

    oxygen_candidate = oxygen_candidates[0]
    co2_candidate = co2_candidates[0]

    return bin_to_dec(oxygen_candidate) * bin_to_dec(co2_candidate)

def transform_input(i):
    return i.splitlines()
