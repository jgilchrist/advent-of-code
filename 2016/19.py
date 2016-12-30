from collections import deque

def part1(number_of_elves):
    elves = list(range(number_of_elves))
    elves = deque(elves)

    while len(elves) > 1:
        front = elves.popleft()
        elves.append(front)
        elves.popleft()

    return elves.popleft()

def part2(number_of_elves):
    middle = (number_of_elves // 2)
    left  = deque(elf for elf in range(0, middle))
    right = deque(elf for elf in range(number_of_elves, middle, -1))

    while left and right:
        if len(left) > len(right):
            left.pop()
        else:
            right.pop()

        right.appendleft(left.popleft())
        left.append(right.pop())

    return left[0] + 1

def transform_input(challenge_input):
    return int(challenge_input)
