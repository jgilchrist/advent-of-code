from utils import *

def react_polymer(polymer):
    s = []

    for char in polymer:
        if not s:
            s.append(char)
            continue

        last_char = s[-1]

        if last_char != char and last_char.upper() == char.upper():
            s.pop()
        else:
            s.append(char)

    return concat(s)

def part1(i):
    return len(react_polymer(i))

def part2(i):
    chars = sorted(set(i))
    middle = len(chars) // 2
    first_half, second_half = chars[:middle], chars[middle:]
    paired_chars = zip(first_half, second_half)

    results = []

    for (c1, c2) in paired_chars:
        new_polymer = str(i)
        new_polymer = new_polymer.replace(c1, '')
        new_polymer = new_polymer.replace(c2, '')

        results.append(len(react_polymer(new_polymer)))

    return min(results)
