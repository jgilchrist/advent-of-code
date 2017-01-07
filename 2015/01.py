def part1(challenge_input):
    number_of_chars = lambda string, char: sum(1 for c in string if c == char)

    up_floors = number_of_chars(challenge_input, '(')
    down_floors = number_of_chars(challenge_input, ')')

    print(up_floors - down_floors)

def part2(challenge_input):
    floor = 0
    for i, c in enumerate(challenge_input, start=1):
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1

        if floor == -1:
            print(i)
            break
