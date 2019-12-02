from utils import *

def part1(program):
    program = run_intcode(program, 12, 2)
    return program[0]

def part2(program):
    for noun in range(100):
        for verb in range(100):
            program_result = run_intcode(program, noun, verb)
            if program_result[0] == 19690720:
                return 100 * noun + verb

    raise Exception("No result found")

def run_intcode(program_to_run, noun, verb):
    program = program_to_run.copy()
    pc = 0
    program[1] = noun
    program[2] = verb

    while True:
        if pc > len(program):
            break

        opcode = program[pc]

        if opcode == 1:
            operandAddr1, operandAddr2, resultAddr = program[pc+1], program[pc+2], program[pc+3]
            operand1, operand2 = program[operandAddr1], program[operandAddr2]
            program[resultAddr] = operand1 + operand2
        elif opcode == 2:
            operandAddr1, operandAddr2, resultAddr = program[pc+1], program[pc+2], program[pc+3]
            operand1, operand2 = program[operandAddr1], program[operandAddr2]
            program[resultAddr] = operand1 * operand2
        else:
            assert opcode == 99
            break

        pc += 4

    return program

def transform_input(i):
    return list(map(int, i.split(',')))
