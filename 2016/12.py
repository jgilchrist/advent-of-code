from collections import namedtuple
from functools import partial
from utils import lmap

PC = 'pc'

def part1(instructions):
    state = { 'pc': 0, 'a': 0, 'b': 0, 'c': 0, 'd': 0 }

    while read(PC, state) < len(instructions):
        current_pc = read(PC, state)
        instruction = instructions[current_pc]
        state = instruction(state)

    print(read('a', state))


def part2(instructions):
    state = { 'pc': 0, 'a': 0, 'b': 0, 'c': 1, 'd': 0 }

    while read(PC, state) < len(instructions):
        current_pc = read(PC, state)
        instruction = instructions[current_pc]
        state = instruction(state)

    print(read('a', state))


def cpy(source, register, state):
    value = read(source, state)
    new_state = set_register(state, register, value)
    return increment_pc(new_state)

def inc(register, state):
    new_value = read(register, state) + 1
    new_state = set_register(state, register, new_value)
    return increment_pc(new_state)

def dec(register, state):
    new_value = read(register, state) - 1
    new_state = set_register(state, register, new_value)
    return increment_pc(new_state)

def jnz(register, amount, state):
    value = read(register, state)

    if value != 0:
        current_pc = read(PC, state)
        return set_register(state, PC, current_pc + amount)
    else:
        return increment_pc(state)

def read(source, state):
    try:
        value = int(source)
        return value
    except ValueError:
        value = state[source]
        return value

def set_register(state, register, value):
    new_state = dict(state)
    new_state[register] = value
    return new_state

def increment_pc(state):
    current_pc = read(PC, state)
    return set_register(state, PC, current_pc + 1)

def transform_input(challenge_input):
    def parse_instruction(instruction):
        opcode = instruction[0]

        if opcode == "cpy":
            (_, source, dest) = instruction
            return partial(cpy, source, dest)
        elif opcode == "inc":
            (_, register) = instruction
            return partial(inc, register)
        elif opcode == "dec":
            (_, register) = instruction
            return partial(dec, register)
        elif opcode == "jnz":
            (_, register, amount) = instruction
            return partial(jnz, register, int(amount))

    instructions = challenge_input.splitlines()
    instructions = [i.split() for i in instructions]
    instructions = lmap(parse_instruction, instructions)
    return instructions
