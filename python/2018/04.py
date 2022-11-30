from utils import *

from collections import defaultdict


@check(99911)
def part1(minutes_asleep):
    minutes_asleep_for_guard = lambda guard: sum(minutes_asleep[guard].values())

    most_sleepy_guard = max(minutes_asleep, key=minutes_asleep_for_guard)

    most_sleepy_minute = max(minutes_asleep[most_sleepy_guard], key=lambda minute: minutes_asleep[most_sleepy_guard][minute])

    return most_sleepy_guard * most_sleepy_minute

@check(65854)
def part2(minutes_asleep):
    sleepiest_minute_per_guard = {
        guard: argmax(minutes_asleep[guard])
        for guard in minutes_asleep
    }

    sleepiest_guard_entry = argmax(sleepiest_minute_per_guard, key=lambda item: item[1][1])
    guard, (minute, times) = sleepiest_guard_entry

    return guard * minute

def transform_input(i):
    def get_line_action(line):
        (year, month, day, hour, minute, *rest) = get_all_positive_numbers(line)
        action = line.split(']')[1].strip()

        return (minute, action)

    def get_sleep_ranges(actions):
        current_guard = None
        sleep_ranges = defaultdict(list)
        for (minute, line), (next_minute, next_line) in window(actions, n=2):
            if 'begins shift' in line:
                (current_guard,) = get_all_numbers(line)

            if 'falls asleep' in line:
                sleep_ranges[current_guard].append((minute, next_minute))

        return sleep_ranges

    def build_minutes_asleep(ranges):
        minutes_asleep = defaultdict(lambda: defaultdict(int))

        for guard, guard_ranges in ranges.items():
            for start_range, end_range in guard_ranges:
                for minute in range(start_range, end_range):
                    minutes_asleep[guard][minute] += 1

        return minutes_asleep

    lines = i.splitlines()
    lines = sorted(lines)
    actions = lmap(get_line_action, lines)

    sleep_ranges = get_sleep_ranges(actions)
    minutes_asleep = build_minutes_asleep(sleep_ranges)
    return minutes_asleep
