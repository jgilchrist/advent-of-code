from utils import *

op = var = '(\w+)'
number = '(-?\d+)'
boolop = '(\S+)'
pattern = f'{var} {op} {number} if {var} {boolop} {number}'

Instruction = namedtuple('Instruction', 'register op amount condition_register condition condition_amount')

ops = {
    'inc': lambda a, b: a + b,
    'dec': lambda a, b: a - b,
}

conditions = {
    '==': lambda a, b: a == b,
    '!=': lambda a, b: a != b,
    '>=': lambda a, b: a >= b,
    '<=': lambda a, b: a <= b,
    '>':  lambda a, b: a > b,
    '<':  lambda a, b: a < b,
}

def part1(i):
    state = defaultdict(int)

    for instruction in i:
        register_value = state[instruction.register]
        condition_register_value = state[instruction.condition_register]

        if instruction.condition(condition_register_value, instruction.condition_amount):
            state[instruction.register] = instruction.op(register_value, instruction.amount)

    return max(state.values())


def part2(i):
    state = defaultdict(int)
    max_value = 0

    for instruction in i:
        register_value = state[instruction.register]
        condition_register_value = state[instruction.condition_register]

        if instruction.condition(condition_register_value, instruction.condition_amount):
            result = instruction.op(register_value, instruction.amount)
            state[instruction.register] = result
            max_value = max(max_value, result)

    return max_value

def transform_input(i):
    lines = i.splitlines()
    lines = [re.search(pattern, line).groups() for line in lines]
    transform = lambda a, b, c, d, e, f: (a, ops[b], int(c), d, conditions[e], int(f))
    lines = [transform(*line) for line in lines]
    return [Instruction(*line) for line in lines]
