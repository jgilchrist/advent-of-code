from utils import *

from math import atan2, pi

def angle_to(c1, c2):
    c2 = tuple_sub(c2, c1)
    return (atan2(*c2) - (pi) + (2 * pi)) % (2 * pi)

@check(278)
def part1(coords):
    base_stations = {}

    for possible_base_station in sorted(coords):
        other_asteroids = coords - set([possible_base_station])

        asteroids_at_angle = defaultdict(list)
        for asteroid in other_asteroids:
            angle = angle_to(possible_base_station, asteroid)
            asteroids_at_angle[angle].append(asteroid)

        seen_asteroids = len(asteroids_at_angle)
        base_stations[possible_base_station] = seen_asteroids

    best_base_station, best_seen_asteroids = argmax(base_stations)
    return best_seen_asteroids

def get_explosions(base_station, asteroids_at_angle):
    keys = cycle(reversed(sorted(asteroids_at_angle.keys())))

    for angle in keys:
        if all(not value for value in asteroids_at_angle.values()):
            return

        if not asteroids_at_angle[angle]: continue

        distances = {
            asteroid: manhattan_distance(base_station, asteroid)
            for asteroid in asteroids_at_angle[angle]
        }

        next_to_explode, _ = argmin(distances)
        asteroids_at_angle[angle].remove(next_to_explode)
        yield next_to_explode

@check(1417)
def part2(coords):
    current_angle = pi
    base_station = (23, 19)

    other_asteroids = coords - set([base_station])

    asteroids_at_angle = defaultdict(list)
    for asteroid in other_asteroids:
        angle = angle_to(base_station, asteroid)
        asteroids_at_angle[angle].append(asteroid)

    # Ensure 0 radians is interpreted as 2*pi radians for ordering
    if 0.0 in asteroids_at_angle:
        asteroids_at_angle[2 * pi] = asteroids_at_angle[0.0]
        del asteroids_at_angle[0.0]

    explosions = list(get_explosions(base_station, asteroids_at_angle))
    two_hundredth_explosion = explosions[199]
    x, y = two_hundredth_explosion
    return x * 100 + y

def transform_input(i):
    coords = set()

    for y, line in enumerate(i.splitlines()):
        for x, char in enumerate(line):
            if char == '#':
                coords.add((x, y))

    return coords
