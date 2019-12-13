from utils import *

from .intcode import IntcodeVm

@check(315)
def part1(program):
    vm = IntcodeVm(program)

    tiles = set()

    while not vm.halted:
        x = vm.run_until_next_output()
        y = vm.run_until_next_output()
        c = vm.run_until_next_output()
        tiles.add((x, y, c))

    return len([c for x,y,c in tiles if c == 2])

@check(16171)
def part2(program):
    program = program.copy()
    program[0] = 2 # Play for free!
    vm = IntcodeVm(program)

    score = 0
    paddle_x = 0
    ball_x = 0
    vm.set_input([0])

    while not vm.halted:
        x = vm.run_until_next_output()
        y = vm.run_until_next_output()
        c = vm.run_until_next_output()

        if x == -1 and y == 0:
            score = c
        elif c == 3:
            paddle_x = x
        elif c == 4:
            ball_x = x

        if ball_x < paddle_x:
            vm.set_input([-1])
        elif ball_x > paddle_x:
            vm.set_input([1])
        else:
            vm.set_input([0])

    return score

def transform_input(i):
    return list(map(int, i.split(',')))
