from utils import *

def part1(i):
    instructions = list(i)
    pc = 0
    steps = 0

    while True:
        instruction = instructions[pc]
        instructions[pc] += 1
        pc += instruction
        steps += 1

        if pc < 0 or pc >= len(instructions):
            break

    return steps

def part2(i):
    instructions = list(i)
    pc = 0
    steps = 0

    while True:
        instruction = instructions[pc]

        if instruction >= 3:
            instructions[pc] -= 1
        else:
            instructions[pc] += 1

        pc += instruction
        steps += 1

        if pc < 0 or pc >= len(instructions):
            break

    return steps

def transform_input(i):
    return lmap(int, i.splitlines())
