from challenge_utils import *

from functools import partial

def challenge_test_runner(year, challenge_number, part1_answer, part2_answer):
    challenge, challenge_input = get_challenge(year, challenge_number)
    part1_output = challenge.part1(challenge_input)
    assert part1_output == part1_answer

    challenge, challenge_input = get_challenge(year, challenge_number)
    part2_output = challenge.part2(challenge_input)
    assert part2_output == part2_answer

def get_test_runner_for_year(year):
    return partial(challenge_test_runner, year)
