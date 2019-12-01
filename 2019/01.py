from utils import *

def part1(i):
    return sum(fuel_required_for_cargo(mass) for mass in i)

def part2(i):
    return sum(fuel_required_for_cargo_and_fuel(mass) for mass in i)

def fuel_required_for_cargo(cargo):
    return max(cargo // 3 - 2, 0)

def fuel_required_for_cargo_and_fuel(cargo):
    fuel = fuel_required_for_cargo(cargo)
    if fuel <= 0: return fuel

    fuel_required_for_fuel = fuel_required_for_cargo_and_fuel(fuel)
    return fuel + fuel_required_for_fuel

def transform_input(i):
    return [int(line) for line in i.splitlines()]
