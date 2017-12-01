import hashlib
import operator
import re

from collections import Counter, deque, namedtuple
from enum import Enum
from functools import reduce, partial
from heapq import heappush, heappop
from itertools import *

def tuple_add(a, b):
    return tuple(map(operator.add, a, b))

def tuple_mul(tup, n):
    return tuple(i * n for i in tup)

def pair_is_inverse(p1, p2):
    (p2x, p2y) = p2
    return p1 == (p2y, p2x)

def transpose(list_of_lists):
    return map(list, zip(*list_of_lists))

def flatten(list_of_lists):
    return list(chain.from_iterable(list_of_lists))

def chunk_list(l, chunk_size):
    for i in range(0, len(l), chunk_size):
        yield l[i:i+chunk_size]

def window(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result

def concat(iterable):
    return "".join(iterable)

def neighbors4(position):
    (x, y) = position
    return [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]

def astar_search(initial_state, heuristic_fn, generator_fn):

    def Path(previous, s):
        "A list of states which lead to state s"
        return ([] if (s is None) else Path(previous, previous[s]) + [s])

    next_states = [(heuristic_fn(initial_state), initial_state)]
    previous    = { initial_state: None }
    path_cost   = { initial_state: 0 }

    while next_states:
        (f, state) = heappop(next_states)

        if heuristic_fn(state) == 0:
            return Path(previous, state)

        for next_state in generator_fn(state):
            new_cost = path_cost[state] + 1

            if next_state not in path_cost or new_cost < path_cost[next_state]:
                heappush(next_states, (new_cost + heuristic_fn(next_state), next_state))
                path_cost[next_state] = new_cost
                previous[next_state] = state

def apply_fn(state, fn):
    return fn(state)

def hash_utf(string):
    m = hashlib.md5()
    m.update(string.encode('utf-8'))
    return m.hexdigest()

def import_challenge(challenge_number):
    return __import__(str(challenge_number).zfill(2))

def read_input(challenge_number):
    challenge = import_challenge(challenge_number)

    filename = "inputs/{0:02d}.input".format(challenge_number)

    with open(filename) as f:
        challenge_input = f.read().strip()

    if hasattr(challenge, 'transform_input'):
        challenge_input = challenge.transform_input(challenge_input)

    return challenge_input

def Part1(challenge_number, answer, skip=False):
    import unittest

    def test_part1(self):
        """Part 1"""
        challenge = import_challenge(challenge_number)
        challenge_input = read_input(challenge_number)
        result = challenge.part1(challenge_input)
        self.assertEqual(result, answer)

    def wrap(cls):
        cls.test_part1 = test_part1
        if skip: cls.test_part1 = unittest.skip("Skipped Part 1")(cls.test_part1)
        return cls

    return wrap

def Part2(challenge_number, answer, skip=False):
    import unittest

    def test_part2(self):
        """Part 2"""
        challenge = import_challenge(challenge_number)
        challenge_input = read_input(challenge_number)
        result = challenge.part2(challenge_input)
        self.assertEqual(result, answer)

    def wrap(cls):
        cls.test_part2 = test_part2
        if skip: cls.test_part2 = unittest.skip("Skipped Part 2")(cls.test_part2)
        return cls

    return wrap
