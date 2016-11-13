from collections import Counter

def transform_input(challenge_input): return challenge_input.strip()

def part1(i):
    pos = (0, 0)
    visited = Counter([pos])

    for char in i:
        pos = move(pos, char)
        visited.update([pos])

    print(len(visited))

def part2(i):
    santa_pos = (0, 0)
    robo_santa_pos = (0, 0)
    visited = Counter([santa_pos])
    santas_turn = True

    for char in i:
        relevant_pos = santa_pos if santas_turn else robo_santa_pos
        relevant_pos = move(relevant_pos, char)
        visited.update([relevant_pos])

        if santas_turn:
            santa_pos = relevant_pos
        else:
            robo_santa_pos = relevant_pos

        santas_turn = not santas_turn

    print(len(visited))


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
