from utils import *

@check(373160)
def part1(i):
    return run(i, lambda instruction: 1)

@check(26395586)
def part2(i):
    return run(i, lambda instruction: -1 if instruction >= 3 else 1)

def run(i, instruction_diff_fn):
    instructions = list(i)
    pc = 0
    steps = 0

    while True:
        instruction = instructions[pc]
        instructions[pc] += instruction_diff_fn(instruction)
        pc += instruction
        steps += 1

        if pc < 0 or pc >= len(instructions):
            break

    return steps

def transform_input(i):
    return lmap(int, i.splitlines())
