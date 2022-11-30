from utils import *

def run_program(program):
    pc = 0
    acc = 0
    seen = set()

    while pc not in seen:
        if pc >= len(program):
            return (True, acc)

        seen.add(pc)
        opcode, value = program[pc]

        if opcode == "nop":
            pc += 1
        elif opcode == "acc":
            acc += value
            pc += 1
        elif opcode == "jmp":
            pc += value

    return (False, acc)


@check(1262)
def part1(program):
    finished, acc = run_program(program)
    assert not finished
    return acc

def substitute_jmp(program, jmp_number):
    jmps_seen = 0
    program = list(program)

    for index, instruction in enumerate(program):
        opcode, value = instruction
        if opcode == "jmp":
            jmps_seen += 1

            if jmps_seen == jmp_number:
                program[index] = ("nop", value)

    return program

@check(1643)
def part2(program):
    njumps = len([(i, v) for (i, v) in program if i == "jmp"])

    for n in range(njumps):
        fixed_program = substitute_jmp(program, n)
        finished, acc = run_program(fixed_program)
        if finished:
            return acc

    assert False

def transform_input(i):
    i = [line.split() for line in i.splitlines()]
    i = [(opcode, int(val)) for opcode, val in i]
    return i
