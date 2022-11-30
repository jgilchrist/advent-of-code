from utils import *

import itertools

def adjustment_amount(m1, m2):
    if m1 < m2:
        return 1
    elif m1 > m2:
        return -1
    else:
        return 0

def apply_gravity_between(m1, m2):
    (m1_x, m1_y, m1_z) = m1.position
    (m2_x, m2_y, m2_z) = m2.position

    m1_x_adjustment = adjustment_amount(m1_x, m2_x)
    m1_y_adjustment = adjustment_amount(m1_y, m2_y)
    m1_z_adjustment = adjustment_amount(m1_z, m2_z)

    m2_x_adjustment = -m1_x_adjustment
    m2_y_adjustment = -m1_y_adjustment
    m2_z_adjustment = -m1_z_adjustment

    m1_adjustment = (m1_x_adjustment, m1_y_adjustment, m1_z_adjustment)
    m2_adjustment = (m2_x_adjustment, m2_y_adjustment, m2_z_adjustment)

    m1.velocity = tuple_add(m1.velocity, m1_adjustment)
    m2.velocity = tuple_add(m2.velocity, m2_adjustment)

def apply_gravity(moons):
    for x, y in itertools.combinations(moons, r=2):
        apply_gravity_between(x, y)

def apply_velocity(moons):
    for moon in moons:
        moon.position = tuple_add(moon.position, moon.velocity)

def potential_energy(moon):
    return sum(abs(i) for i in moon.position)

def kinetic_energy(moon):
    return sum(abs(i) for i in moon.velocity)

class Moon:
    def __init__(self, name, position):
        self.name = name
        self.position = position
        self.velocity = (0, 0, 0)

    def __repr__(self):
        return f"({self.name})<position={self.position}, velocity={self.velocity}>"

@check(12053)
def part1(moons):
    moons = moons.copy()

    for i in range(1000):
        apply_gravity(moons)
        apply_velocity(moons)

    return sum(potential_energy(moon) * kinetic_energy(moon) for moon in moons)

@check(320380285873116)
def part2(moons):
    def get_period_for_dimension(fn):
        ms = moons.copy()
        seen = set()
        steps = 0

        while True:
            apply_gravity(ms)
            apply_velocity(ms)
            state = fn(ms)

            if state in seen:
                return steps

            steps += 1
            seen.add(state)

    x_period = get_period_for_dimension(lambda ms: (tuple(m.position[0] for m in ms), tuple(m.velocity[0] for m in ms)))
    y_period = get_period_for_dimension(lambda ms: (tuple(m.position[1] for m in ms), tuple(m.velocity[1] for m in ms)))
    z_period = get_period_for_dimension(lambda ms: (tuple(m.position[2] for m in ms), tuple(m.velocity[2] for m in ms)))

    return lcm(x_period, lcm(y_period, z_period))

def transform_input(i):
    numbers = [list(get_all_numbers(line)) for line in i.splitlines()]
    moons = [Moon(i, p) for i, p in enumerate(numbers)]
    return moons
