from utils import *

def all_multipliers(num):
    curr = num

    while True:
        yield curr
        curr += num

def first_departure_time(bus, departure_time):
    return next(n for n in all_multipliers(bus) if n >= departure_time)

@check(333)
def part1(i):
    departure_time, buses = i
    buses = [bus for bus in buses if bus is not None]

    best_departure_times = {
        bus: first_departure_time(bus, departure_time)
        for bus in buses
    }

    best_bus, best_bus_departure_time = argmin(best_departure_times)

    return best_bus * (best_bus_departure_time - departure_time)

@check(690123192779524)
def part2(i):
    _, buses = i
    buses = [(bus, idx) for (idx, bus) in enumerate(buses) if bus is not None]

    timestamp = 0
    step = 1

    for (bus, idx) in buses:
        timestamp = next(num for num in count(timestamp, step) if (num + idx) % bus == 0)
        step *= bus

    return timestamp

def transform_input(i):
    departure_time, buses = i.splitlines()
    departure_time = int(departure_time)
    buses = [int(bus) if bus != "x" else None for bus in buses.split(',')]
    return departure_time, buses
