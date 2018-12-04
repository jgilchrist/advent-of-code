#!/usr/bin/env python3

from challenge_utils import *

class Color:
    Red = '\033[91m'
    Yellow = '\033[93m'
    Green = '\033[92m'
    Blue = '\033[94m'
    Reset = '\033[0m'

def red(t): return f'{Color.Red}{t}{Color.Reset}'
def yellow(t): return f'{Color.Yellow}{t}{Color.Reset}'
def green(t): return f'{Color.Green}{t}{Color.Reset}'
def blue(t): return f'{Color.Blue}{t}{Color.Reset}'

def run_challenge(year, challenge_number):
    challenge, challenge_input = get_challenge(year, challenge_number)

    print(f'{red("=")}{green("=")} Challenge {yellow(challenge_number)}')

    print(f'{red("1")}: ', end="")
    print(challenge.part1(challenge_input))

    print(f'{green("2")}: ', end="")
    print(challenge.part2(challenge_input))

if __name__ == '__main__':
    challenges = range(1, 26)

    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("year", nargs='?', type=str, default='2018')
    parser.add_argument("challenges", type=int, nargs='*', default=challenges)
    args = parser.parse_args()

    print(f'Running challenges from year {yellow(args.year)}')

    for c in args.challenges:
        if c not in challenges:
            print(f'{red("Error")}: Challenge {c} does not exist')
            continue

        run_challenge(args.year, c)
