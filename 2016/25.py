from utils import *

PC = 'pc'

@check(198)
@slow
def part1(instructions):
    # We will call the program a success if the first 10 characters of output
    # are 01010101...
    expected_stdout = '0101010101'

    for a_candidate in count(1):
        state = { 'pc': 0, 'a': a_candidate, 'b': 0, 'c': 0, 'd': 0 }
        stdout = ''

        while read(PC, state) < len(instructions):
            current_pc = read(PC, state)
            instruction = instructions[current_pc]
            (state, new_char) = instruction(state)

            if new_char is not None:
                stdout += str(new_char)

            if len(stdout) == 10:
                if stdout == expected_stdout:
                    return a_candidate
                else:
                    break

@check(None)
def part2(instructions):
    print("Merry Christmas!")
    return

def cpy(source, register, state):
    value = read(source, state)
    new_state = set_register(state, register, value)
    return increment_pc(new_state), None

def inc(register, state):
    new_value = read(register, state) + 1
    new_state = set_register(state, register, new_value)
    return increment_pc(new_state), None

def dec(register, state):
    new_value = read(register, state) - 1
    new_state = set_register(state, register, new_value)
    return increment_pc(new_state), None

def jnz(register, amount, state):
    value = read(register, state)

    if value != 0:
        current_pc = read(PC, state)
        return set_register(state, PC, current_pc + amount), None
    else:
        return increment_pc(state), None

def out(register, state):
    return increment_pc(state), read(register, state)

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
        elif opcode == "out":
            (_, register) = instruction
            return partial(out, register)

    instructions = challenge_input.splitlines()
    instructions = [i.split() for i in instructions]
    instructions = list(map(parse_instruction, instructions))
    return instructions
