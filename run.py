#!/usr/bin/env python3

import importlib
import time

class Color:
    Grey = '\033[90m'
    Red = '\033[91m'
    Yellow = '\033[93m'
    Green = '\033[92m'
    Blue = '\033[94m'
    Reset = '\033[0m'

def grey(t): return f'{Color.Grey}{t}{Color.Reset}'
def red(t): return f'{Color.Red}{t}{Color.Reset}'
def yellow(t): return f'{Color.Yellow}{t}{Color.Reset}'
def green(t): return f'{Color.Green}{t}{Color.Reset}'
def blue(t): return f'{Color.Blue}{t}{Color.Reset}'

current_milli_time = lambda: int(round(time.time() * 1000))

def import_challenge(year, challenge_number):
    return importlib.import_module(f'{year}.{challenge_number:02d}')

def read_input(year, challenge_number):
    filename = f'{year}/{challenge_number:02d}.in'

    with open(filename) as f:
        challenge_input = f.read().strip()

    return challenge_input

def get_challenge(year, challenge_number):
    try:
        challenge_input = read_input(year, challenge_number)
    except FileNotFoundError:
        # No input for this challenge yet - skip it
        return (None, None)

    try:
        challenge = import_challenge(year, challenge_number)
    except ImportError as e:
        if "No module named" in str(e):
            print(f'{red("Error")}: No solution for challenge {challenge_number:02d}')
            return (None, None)
        else:
            print(e)
            return (None, None)

    if hasattr(challenge, 'transform_input'):
        challenge_input = challenge.transform_input(challenge_input)

    return (challenge, challenge_input)

def print_solution(solution, expected_solution, duration):
    print(solution, end="")

    print(grey(' ('), end="")

    if expected_solution is not None:
        if solution == expected_solution:
            print(f'{green("+")}, ', end="")
        else:
            print(f'{red("-")}, ', end="")
    else:
        print(f'{grey("?")}, ', end="")

    duration_color_fn = green
    if duration > 1000:
        duration_color_fn = yellow
    if duration > 10000:
        duration_color_fn = red

    print(duration_color_fn(f'{duration}ms'), end="")
    print(grey(')'))


def run_challenge(year, challenge_number, run_slow_challenges):
    challenge, challenge_input = get_challenge(year, challenge_number)

    challenge_text = f'Challenge {yellow(challenge_number)}' if challenge is not None else grey(f"Challenge {challenge_number}")

    print(f'{red("=")}{green("=")} {challenge_text}')

    if challenge is None:
        return

    if hasattr(challenge, 'test'):
        challenge.test()

    print(f'{red("1")}: ', end="")

    if hasattr(challenge.part1, 'is_slow') and not run_slow_challenges:
        print(yellow('skipped (slow)'))
    else:
        c1_start = current_milli_time()
        c1_result = challenge.part1(challenge_input)
        c1_end = current_milli_time()
        c1_duration = c1_end - c1_start
        print_solution(
            c1_result,
            challenge.part1.solution if hasattr(challenge.part1, 'solution') else None,
            c1_duration
        )

    print(f'{green("2")}: ', end="")

    if hasattr(challenge.part2, 'is_slow') and not run_slow_challenges:
        print(yellow('skipped (slow)'))
    else:
        c2_start = current_milli_time()
        c2_result = challenge.part2(challenge_input)
        c2_end = current_milli_time()
        c2_duration = c2_end - c2_start
        print_solution(
            c2_result,
            challenge.part2.solution if hasattr(challenge.part2, 'solution') else None,
            c2_duration
        )

if __name__ == '__main__':
    challenges = range(1, 26)

    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("year", nargs='?', type=str, default='2020')
    parser.add_argument("challenges", type=int, nargs='*', default=challenges)
    parser.add_argument("--run-slow-challenges", required=False, action='store_true')
    args = parser.parse_args()

    print(f'Running challenges from year {yellow(args.year)}')

    for c in args.challenges:
        if c not in challenges:
            print(f'{red("Error")}: Challenge {c} does not exist')
            continue

        run_challenge(args.year, c, args.run_slow_challenges)
