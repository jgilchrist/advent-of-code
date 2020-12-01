from utils import *

@check(346386)
def part1(challenge_input):
    for i, hash_string in enumerate(all_hashes(challenge_input)):
        if hash_string.startswith('00000'):
            return i

@check(9958218)
@slow
def part2(challenge_input):
    for i, hash_string in enumerate(all_hashes(challenge_input)):
        if hash_string.startswith('000000'):
            return i

def all_hashes(challenge_input):
    natural_numbers_as_strings = map(str, count())
    input_with_numbers = zip(repeat(challenge_input), natural_numbers_as_strings)
    input_strings = map(concat, input_with_numbers)

    # An infinite generator of all hashed inputs
    return map(hash_utf, input_strings)
