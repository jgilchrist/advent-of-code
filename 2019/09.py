from utils import *

class IntcodeState:
    def __init__(self, program, inputs = None):
        self.program = program.copy()
        self.pc = 0
        self.halted = False
        self.relative_base = 0

        if inputs is None:
            self.inputs = []
        else:
            self.inputs = inputs

    def adjust_relative_base(self, base):
        self.relative_base += base

    def get_value(self, addressOrValue, mode):
        assert mode == 0 or mode == 1 or mode == 2
        if mode == 1: return addressOrValue

        address = addressOrValue + self.relative_base if mode == 2 else addressOrValue

        while len(self.program) <= address:
            self.program.append(0)

        return self.program[address]

    def set_value(self, address, value, mode=0):
        assert mode == 0 or mode == 2

        address = address + self.relative_base if mode == 2 else address

        while len(self.program) <= address:
            self.program.append(0)

        self.program[address] = value

    def add_input(self, value):
        self.inputs.append(value)

    def next_input(self):
        if not self.inputs:
            raise Exception("Tried to read from empty input list")

        value = self.inputs[0]
        self.inputs.pop(0)
        return value


def run_intcode_until_output(state):
    def get_next():
        value = state.program[state.pc]
        return state.pc + 1, value


    while True:
        if state.pc > len(state.program):
            break

        state.pc, opcode = get_next()
        opcode = str(opcode).zfill(5)

        [*param_modes, opcodeBit1, opcodeBit2] = opcode
        opcode = int(opcodeBit1 + opcodeBit2)

        # Param modes are from right to left
        param_modes = list(map(int, reversed(param_modes)))

        if opcode == 1:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = state.get_value(operand1, param_modes[0]), state.get_value(operand2, param_modes[1])
            state.set_value(operand3, operand1 + operand2, param_modes[2])
        elif opcode == 2:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = state.get_value(operand1, param_modes[0]), state.get_value(operand2, param_modes[1])
            state.set_value(operand3, operand1 * operand2, param_modes[2])
        elif opcode == 3:
            state.pc, operand = get_next()
            next_input = state.next_input()
            state.set_value(operand, next_input, param_modes[0])
        elif opcode == 4:
            state.pc, operand = get_next()
            valueToOutput = state.get_value(operand, param_modes[0])
            return valueToOutput
        elif opcode == 5:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            operand1, operand2 = state.get_value(operand1, param_modes[0]), state.get_value(operand2, param_modes[1])
            if operand1 != 0:
                state.pc = operand2
        elif opcode == 6:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            operand1, operand2 = state.get_value(operand1, param_modes[0]), state.get_value(operand2, param_modes[1])
            if operand1 == 0:
                state.pc = operand2
        elif opcode == 7:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = state.get_value(operand1, param_modes[0]), state.get_value(operand2, param_modes[1])
            state.set_value(operand3, int(operand1 < operand2), param_modes[2])
        elif opcode == 8:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = state.get_value(operand1, param_modes[0]), state.get_value(operand2, param_modes[1])
            state.set_value(operand3, int(operand1 == operand2), param_modes[2])
        elif opcode == 9:
            state.pc, operand1 = get_next()
            operand1 = state.get_value(operand1, param_modes[0])
            state.adjust_relative_base(operand1)
        else:
            assert opcode == 99
            state.halted = True
            return None

    return output

def part1(program):
    state = IntcodeState(program, inputs=[1])

    outputs = []

    while not state.halted:
        output = run_intcode_until_output(state)
        if output is not None:
            outputs.append(output)

    return outputs[-1]

def part2(program):
    state = IntcodeState(program, inputs=[2])

    outputs = []

    while not state.halted:
        output = run_intcode_until_output(state)
        if output is not None:
            outputs.append(output)

    return outputs[-1]

def transform_input(i):
    return list(map(int, i.split(',')))
