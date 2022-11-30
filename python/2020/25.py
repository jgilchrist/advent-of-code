from utils import *

def loop_values(subject_number):
    start = 1

    while True:
        yield start
        start *= subject_number
        start = start % 20201227

def loop_value(subject_number, n):
    return next(val for i, val in enumerate(loop_values(subject_number)) if i == n)

def get_loop_size(val):
    return next(i for i, n in enumerate(loop_values(7)) if n == val)

@check(6421487)
def part1(i):
    card_public_key, door_public_key = i
    card_loop_size, door_loop_size = get_loop_size(card_public_key), get_loop_size(door_public_key)
    e1, e2 = loop_value(door_public_key, card_loop_size), loop_value(card_public_key, door_loop_size)
    return e1

@check(None)
def part2(i):
    print("Merry Christmas!")

def transform_input(i):
    line1, line2 = i.splitlines()
    return int(line1), int(line2)
