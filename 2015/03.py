def part1(i):
    pos = (0, 0)
    visited = set([pos])

    for char in i:
        pos = move(pos, char)
        visited.add(pos)

    return len(visited)

def part2(i):
    santa = (0, 0)
    robo_santa = (0, 0)
    visited = set([santa])

    santas_instructions = i[::2]
    robo_santas_instructions = i[1::2]

    for char in santas_instructions:
        santa = move(santa, char)
        visited.add(santa)

    for char in robo_santas_instructions:
        robo_santa = move(robo_santa, char)
        visited.add(robo_santa)

    return len(visited)


def move(pos, direction):
    (x, y) = pos

    if direction == '^':
        return (x, y-1)
    elif direction == 'v':
        return (x, y+1)
    elif direction == '<':
        return (x-1, y)
    elif direction == '>':
        return (x+1, y)
