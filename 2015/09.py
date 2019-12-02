from utils import *

import itertools

def calculate_distance(visit_list, distances):
    return sum(distances[(a, b)] for a, b in window(visit_list, 2))

def calculate_all_journey_distances(distances):
    keys = distances.keys()
    cities = set(a for (a,b) in keys) | set(b for (a,b) in keys)
    distances = [calculate_distance(visit, distances) for visit in itertools.permutations(cities)]
    distances = [d for d in distances if d]
    return distances

def part1(i):
    return min(calculate_all_journey_distances(i))

def part2(i):
    return max(calculate_all_journey_distances(i))

def transform_input(i):
    def parse_line(line):
        match = re.search("^(\w+) to (\w+) = (\d+)$", line)
        from_city = match.group(1)
        to_city  = match.group(2)
        distance = int(match.group(3))
        return (from_city, to_city, distance)

    lines = [parse_line(line) for line in i.splitlines()]

    distances = {}
    for from_city, to_city, distance in lines:
        distances[(from_city, to_city)] = distance
        distances[(to_city, from_city)] = distance

    return distances
