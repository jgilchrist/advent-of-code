import hashlib
import importlib
import operator
import re

from collections import Counter, defaultdict, deque, namedtuple
from enum import Enum
from functools import reduce, partial
from heapq import heappush, heappop
from itertools import *

def lmap(fn, seq):
    return list(map(fn, seq))

def tuple_add(a, b):
    return tuple(map(operator.add, a, b))

def tuple_mul(tup, n):
    return tuple(i * n for i in tup)

def manhattan_distance(tup1, tup2):
    (x1, y1) = tup1
    (x2, y2) = tup2
    return abs(x1 - x2) + abs(y1 - y2)

def transpose(list_of_lists):
    return map(list, zip(*list_of_lists))

def flatten(list_of_lists):
    return list(chain.from_iterable(list_of_lists))

def take(n, iterable):
    return list(islice(iterable, n))

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

digits_regex = re.compile(r'-?\d+')
positive_digits_regex = re.compile(r'\d+')

def get_all_numbers(string):
    return map(int, digits_regex.findall(string))

def get_all_positive_numbers(string):
    return map(int, positive_digits_regex.findall(string))

def argmax(dictionary, key=None):
    if key is None:
        key = operator.itemgetter(1)
    return max(dictionary.items(), key=key)

def argmin(dictionary, key=None):
    if key is None:
        key = operator.itemgetter(1)
    return min(dictionary.items(), key=key)
