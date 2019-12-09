from utils import *

def play(players, play_until):
    marbles = [0]
    current_marble = 0
    scores = defaultdict(int)

    for i in range(1, play_until+1):
        print(len(marbles))
        current_marble = min((current_marble + 2) % (len(marbles) + 1), len(marbles)+1)
        print(current_marble)
        marbles.insert(current_marble, i)

        print(marbles)



def part1(i):
    return play(7, 25)

def part2(i):
    pass

def transform_input(i):
    return i
