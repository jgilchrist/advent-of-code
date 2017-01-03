from utils import *

PC = 'pc'

def part1(instructions):
    state = { 'pc': 0, 'a': 7, 'b': 0, 'c': 0, 'd': 0 }

    while read(PC, state) < len(instructions):
        current_pc = read(PC, state)
        instruction = instructions[current_pc]
        state, instructions = instruction(state, instructions)

    return read('a', state)


def part2(instructions):
    state = { 'pc': 0, 'a': 12, 'b': 0, 'c': 1, 'd': 0 }

    while read(PC, state) < len(instructions):
        current_pc = read(PC, state)
        instruction = instructions[current_pc]
        state, instructions = instruction(state, instructions)

    return read('a', state)


def cpy(source, register, state, instructions):
    value = read(source, state)
    new_state = set_register(state, register, value)
    return increment_pc(new_state), instructions

def inc(register, state, instructions):
    new_value = read(register, state) + 1
    new_state = set_register(state, register, new_value)
    return increment_pc(new_state), instructions

def dec(register, state, instructions):
    new_value = read(register, state) - 1
    new_state = set_register(state, register, new_value)
    return increment_pc(new_state), instructions

def jnz(register, amount, state, instructions):
    value = read(register, state)
    numeric_amount = read(amount, state)

    if value != 0:
        current_pc = read(PC, state)
        return set_register(state, PC, current_pc + numeric_amount), instructions
    else:
        return increment_pc(state), instructions

def tgl(register, state, instructions):
    instruction_pos = read(PC, state) + read(register, state)

    # Do nothing if attempting to toggle outside of the program
    if instruction_pos < 0 or instruction_pos >= len(instructions):
        return increment_pc(state), instructions

    i = instructions[instruction_pos]
    args = i.args
    new_func = {
        cpy: jnz,
        inc: dec,
        dec: inc,
        jnz: cpy,
        tgl: inc,
    }[i.func]
    replacement_instruction = partial(new_func, *args)

    new_instructions = list(instructions)
    new_instructions[instruction_pos] = replacement_instruction
    return increment_pc(state), new_instructions

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
            (_, register, register2) = instruction
            return partial(jnz, register, register2)
        elif opcode == "tgl":
            (_, register) = instruction
            return partial(tgl, register)

    instructions = challenge_input.splitlines()
    instructions = [i.split() for i in instructions]
    instructions = list(map(parse_instruction, instructions))
    return instructions
