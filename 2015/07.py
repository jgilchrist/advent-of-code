from utils import *

from enum import Enum

class Op(Enum):
    ASSIGN = 0,
    NOT = 1,
    AND = 2,
    OR = 3,
    LSHIFT = 4,
    RSHIFT = 5

def get(state, var):
    if var.isdigit():
        return int(var)
    else:
        return state[var]

def set_state(state, var, value):
    state[var] = int(value)

def part1(i):
    instrs = list(i)
    state = {}

    while len(instrs) is not 0:
        for instruction in instrs:
            try:
                apply_instruction(instruction, state)
                instrs.remove(instruction)
            except KeyError as e:
                pass

    return state['a']

def part2(i):
    instrs = list(i)
    a_state = part1(instrs)
    i = [instruction for instruction in i if not (instruction[0] == Op.ASSIGN and instruction[2] == 'b')]
    i = [(Op.ASSIGN, str(a_state), 'b')] + i
    return part1(i)

def apply_instruction(instruction, state):
    opcode = instruction[0]

    if opcode == Op.ASSIGN:
        opcode, source, dest = instruction
        source = get(state, source)
        set_state(state, dest, source)
    elif opcode == Op.NOT:
        opcode, source, dest = instruction
        source = get(state, source)
        set_state(state, dest, ~source)
    elif opcode == Op.AND:
        opcode, param1, param2, dest = instruction
        param1 = get(state, param1)
        param2 = get(state, param2)
        set_state(state, dest, param1 & param2)
    elif opcode == Op.OR:
        opcode, param1, param2, dest = instruction
        param1 = get(state, param1)
        param2 = get(state, param2)
        set_state(state, dest, param1 | param2)
    elif opcode == Op.LSHIFT:
        opcode, param1, param2, dest = instruction
        param1 = get(state, param1)
        param2 = get(state, param2)
        set_state(state, dest, param1 << param2)
    elif opcode == Op.RSHIFT:
        opcode, param1, param2, dest = instruction
        param1 = get(state, param1)
        param2 = get(state, param2)
        set_state(state, dest, param1 >> param2)
    else:
        exit(f'Unknown instruction: {instruction}')

def transform_input(i):
    fns = [
        ('(\w+) -> (\w+)', lambda p1, p2: (Op.ASSIGN, p1, p2)),
        ('NOT (\w+) -> (\w+)', lambda p1, p2: (Op.NOT, p1, p2)),
        ('(\w+) AND (\w+) -> (\w+)', lambda p1, p2, p3: (Op.AND, p1, p2, p3)),
        ('(\w+) OR (\w+) -> (\w+)', lambda p1, p2, p3: (Op.OR, p1, p2, p3)),
        ('(\w+) LSHIFT (\w+) -> (\w+)', lambda p1, p2, p3: (Op.LSHIFT, p1, p2, p3)),
        ('(\w+) RSHIFT (\w+) -> (\w+)', lambda p1, p2, p3: (Op.RSHIFT, p1, p2, p3)),
    ]

    def function_for(instruction):
        for (regex, fn) in fns:
            if re.match(regex, instruction):
                groups = re.search(regex, instruction).groups()
                return fn(*groups)

    return lmap(function_for, i.splitlines())
