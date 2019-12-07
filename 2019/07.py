from utils import *

class IntcodeState:
    def __init__(self, program, inputs = None):
        self.program = program.copy()
        self.pc = 0
        self.halted = False

        if inputs is None:
            self.inputs = []
        else:
            self.inputs = inputs

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

    def get_value(state, addressOrValue, mode):
        assert mode == 0 or mode == 1
        if mode == 1: return addressOrValue
        return state.program[addressOrValue]

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
            operand1, operand2 = get_value(state, operand1, param_modes[0]), get_value(state, operand2, param_modes[1])
            state.program[operand3] = operand1 + operand2
        elif opcode == 2:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = get_value(state, operand1, param_modes[0]), get_value(state, operand2, param_modes[1])
            state.program[operand3] = operand1 * operand2
        elif opcode == 3:
            state.pc, operand = get_next()
            next_input = state.next_input()
            state.program[operand] = next_input
        elif opcode == 4:
            state.pc, operand = get_next()
            valueToOutput = state.program[operand]
            return valueToOutput
        elif opcode == 5:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            operand1, operand2 = get_value(state, operand1, param_modes[0]), get_value(state, operand2, param_modes[1])
            if operand1 != 0:
                state.pc = operand2
        elif opcode == 6:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            operand1, operand2 = get_value(state, operand1, param_modes[0]), get_value(state, operand2, param_modes[1])
            if operand1 == 0:
                state.pc = operand2
        elif opcode == 7:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = get_value(state, operand1, param_modes[0]), get_value(state, operand2, param_modes[1])
            state.program[operand3] = int(operand1 < operand2)
        elif opcode == 8:
            state.pc, operand1 = get_next()
            state.pc, operand2 = get_next()
            state.pc, operand3 = get_next()
            operand1, operand2 = get_value(state, operand1, param_modes[0]), get_value(state, operand2, param_modes[1])
            state.program[operand3] = int(operand1 == operand2)
        else:
            assert opcode == 99
            state.halted = True
            return None

    return output

def part1(program):
    outputs = {}

    for permutation in permutations((0, 1, 2, 3, 4)):
        last_output = 0

        for i in permutation:
            state = IntcodeState(program, inputs=[i, last_output])
            output = run_intcode_until_output(state)
            last_output = output

        outputs[permutation] = last_output

    best_permutation, best_value = argmax(outputs)
    return best_value

def part2(program):
    outputs = {}

    def run_cycling_intcode(permutation):
        last_output = 0

        p1, p2, p3, p4, p5 = permutation

        states = [
            IntcodeState(program, inputs=[p1,0]),
            IntcodeState(program, inputs=[p2]),
            IntcodeState(program, inputs=[p3]),
            IntcodeState(program, inputs=[p4]),
            IntcodeState(program, inputs=[p5]),
        ]

        for (s1, s2) in window(cycle(states)):
            output = run_intcode_until_output(s1)
            if output is not None:
                last_output = output
                s2.add_input(output)

            if all(s.halted for s in states):
                return last_output

    for permutation in permutations((5, 6, 7, 8, 9)):
        output = run_cycling_intcode(permutation)
        outputs[permutation] = output

    max_permutation, max_value = argmax(outputs)
    return max_value

def transform_input(i):
    return list(map(int, i.split(',')))
