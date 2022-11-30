class IntcodeVm:
    def __init__(self, program, inputs = None):
        self.program = program.copy()
        self.pc = 0
        self.halted = False
        self.relative_base = 0
        self.outputs = []

        if inputs is None:
            self.inputs = []
        else:
            self.inputs = inputs

    def adjust_relative_base(self, base):
        self.relative_base += base

    def get_next(self):
        value = self.program[self.pc]
        return self.pc + 1, value

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

    def set_input(self, value):
        self.inputs = value

    def add_input(self, value):
        self.inputs.append(value)

    def next_input(self):
        if not self.inputs:
            raise Exception("Tried to read from empty input list")

        value = self.inputs[0]
        self.inputs.pop(0)
        return value

    def run_until_halted(self):
        while not self.halted:
            self.run_until_next_output()

    def run_until_next_output(self):
        while not self.halted:
            if self.pc > len(self.program):
                break

            self.pc, opcode = self.get_next()
            opcode = str(opcode).zfill(5)

            [*param_modes, opcodeBit1, opcodeBit2] = opcode
            opcode = int(opcodeBit1 + opcodeBit2)

            # Param modes are from right to left
            param_modes = list(map(int, reversed(param_modes)))

            if opcode == 1:
                self.pc, operand1 = self.get_next()
                self.pc, operand2 = self.get_next()
                self.pc, operand3 = self.get_next()
                operand1, operand2 = self.get_value(operand1, param_modes[0]), self.get_value(operand2, param_modes[1])
                self.set_value(operand3, operand1 + operand2, param_modes[2])
            elif opcode == 2:
                self.pc, operand1 = self.get_next()
                self.pc, operand2 = self.get_next()
                self.pc, operand3 = self.get_next()
                operand1, operand2 = self.get_value(operand1, param_modes[0]), self.get_value(operand2, param_modes[1])
                self.set_value(operand3, operand1 * operand2, param_modes[2])
            elif opcode == 3:
                self.pc, operand = self.get_next()
                next_input = self.next_input()
                self.set_value(operand, next_input, param_modes[0])
            elif opcode == 4:
                self.pc, operand = self.get_next()
                valueToOutput = self.get_value(operand, param_modes[0])
                self.outputs.append(valueToOutput)
                return valueToOutput
            elif opcode == 5:
                self.pc, operand1 = self.get_next()
                self.pc, operand2 = self.get_next()
                operand1, operand2 = self.get_value(operand1, param_modes[0]), self.get_value(operand2, param_modes[1])
                if operand1 != 0:
                    self.pc = operand2
            elif opcode == 6:
                self.pc, operand1 = self.get_next()
                self.pc, operand2 = self.get_next()
                operand1, operand2 = self.get_value(operand1, param_modes[0]), self.get_value(operand2, param_modes[1])
                if operand1 == 0:
                    self.pc = operand2
            elif opcode == 7:
                self.pc, operand1 = self.get_next()
                self.pc, operand2 = self.get_next()
                self.pc, operand3 = self.get_next()
                operand1, operand2 = self.get_value(operand1, param_modes[0]), self.get_value(operand2, param_modes[1])
                self.set_value(operand3, int(operand1 < operand2), param_modes[2])
            elif opcode == 8:
                self.pc, operand1 = self.get_next()
                self.pc, operand2 = self.get_next()
                self.pc, operand3 = self.get_next()
                operand1, operand2 = self.get_value(operand1, param_modes[0]), self.get_value(operand2, param_modes[1])
                self.set_value(operand3, int(operand1 == operand2), param_modes[2])
            elif opcode == 9:
                self.pc, operand1 = self.get_next()
                operand1 = self.get_value(operand1, param_modes[0])
                self.adjust_relative_base(operand1)
            else:
                assert opcode == 99
                self.halted = True
                return None
