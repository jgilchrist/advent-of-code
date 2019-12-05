from utils import *

def run_intcode(program, inputFn):
    program = program.copy()
    pc = 0
    output = []

    def get_next():
        value = program[pc]
        return pc + 1, value

    def get_value(program, addressOrValue, mode):
        assert mode == 0 or mode == 1
        if mode == 1: return addressOrValue
        return program[addressOrValue]

    while True:
        if pc > len(program):
            break

        pc, opcode = get_next()
        opcode = str(opcode).zfill(5)

        [*param_modes, opcodeBit1, opcodeBit2] = opcode
        opcode = int(opcodeBit1 + opcodeBit2)

        # Param modes are from right to left
        param_modes = list(map(int, reversed(param_modes)))

        if opcode == 1:
            pc, operand1 = get_next()
            pc, operand2 = get_next()
            pc, operand3 = get_next()
            operand1, operand2 = get_value(program, operand1, param_modes[0]), get_value(program, operand2, param_modes[1])
            program[operand3] = operand1 + operand2
        elif opcode == 2:
            pc, operand1 = get_next()
            pc, operand2 = get_next()
            pc, operand3 = get_next()
            operand1, operand2 = get_value(program, operand1, param_modes[0]), get_value(program, operand2, param_modes[1])
            program[operand3] = operand1 * operand2
        elif opcode == 3:
            pc, operand = get_next()
            program[operand] = inputFn()
        elif opcode == 4:
            pc, operand = get_next()
            valueToOutput = program[operand]
            output.append(valueToOutput)
        elif opcode == 5:
            pc, operand1 = get_next()
            pc, operand2 = get_next()
            operand1, operand2 = get_value(program, operand1, param_modes[0]), get_value(program, operand2, param_modes[1])
            if operand1 != 0:
                pc = operand2
        elif opcode == 6:
            pc, operand1 = get_next()
            pc, operand2 = get_next()
            operand1, operand2 = get_value(program, operand1, param_modes[0]), get_value(program, operand2, param_modes[1])
            if operand1 == 0:
                pc = operand2
        elif opcode == 7:
            pc, operand1 = get_next()
            pc, operand2 = get_next()
            pc, operand3 = get_next()
            operand1, operand2 = get_value(program, operand1, param_modes[0]), get_value(program, operand2, param_modes[1])
            program[operand3] = int(operand1 < operand2)
        elif opcode == 8:
            pc, operand1 = get_next()
            pc, operand2 = get_next()
            pc, operand3 = get_next()
            operand1, operand2 = get_value(program, operand1, param_modes[0]), get_value(program, operand2, param_modes[1])
            program[operand3] = int(operand1 == operand2)
        else:
            assert opcode == 99
            break

    return output

def part1(i):
    output = run_intcode(i, lambda: 1)
    return output[-1]

def part2(i):
    output = run_intcode(i, lambda: 5)
    return output[-1]

def transform_input(i):
    return list(map(int, i.split(',')))
