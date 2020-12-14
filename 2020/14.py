from utils import *

class Action(Enum):
    MASK = 0
    MEMSET = 1

def bin_to_int(n):
    return int(n, 2)

def mask_with_rule(num, mask, rule_fn):
    # Skip the initial 0b
    bits = format(num, '#038b')[2:]

    bits = "".join([
        rule_fn(bit, mask_bit)
        for (bit, mask_bit) in zip(bits, mask)
    ])

    return "".join(bits)


@check(7440382076205)
def part1(i):
    def run_instruction(instruction, current_mask, memory):
        if instruction[0] == Action.MASK:
            _, new_mask = instruction
            return (new_mask, memory)
        else:
            _, addr, num = instruction
            num = mask_with_rule(num, current_mask, lambda bit, mask_bit: mask_bit if mask_bit != 'X' else bit)
            num = bin_to_int(num)
            memory[addr] = num
            return (current_mask, memory)

    memory = {}
    mask = None

    for instruction in i:
        mask, memory = run_instruction(instruction, mask, memory)

    return sum(memory.values())

@check(4200656704538)
def part2(i):
    def substitute_xs_in_address(addr, substitute_bits):
        for i in substitute_bits:
            addr = addr.replace('X', i, 1)

        return addr

    def expand_addr(addr):
        xs = addr.count('X')
        possible_bit_combinations = list(itertools.product(('0', '1'), repeat=xs))
        return [substitute_xs_in_address(addr, bit_pattern) for bit_pattern in possible_bit_combinations]

    def run_instruction(instruction, current_mask, memory):
        if instruction[0] == Action.MASK:
            _, new_mask = instruction
            return (new_mask, memory)
        else:
            _, addr, num = instruction
            addr = mask_with_rule(addr, current_mask, lambda bit, mask_bit: bit if mask_bit == '0' else mask_bit)
            all_addrs = expand_addr(addr)

            for expanded_addr in all_addrs:
                memory[bin_to_int(expanded_addr)] = num

            return (current_mask, memory)

    memory = {}
    mask = None

    for instruction in i:
        mask, memory = run_instruction(instruction, mask, memory)

    return sum(memory.values())

def transform_input(i):
    def parse_mask_line(line):
        line = line.split(' ')
        mask = line[2]

        return (Action.MASK, list(mask))

    def parse_memset_line(line):
        numbers = list(get_all_positive_numbers(line))
        return (Action.MEMSET, numbers[0], numbers[1])

    def parse_line(line):
        if line.startswith("mask"):
            return parse_mask_line(line)
        else:
            return parse_memset_line(line)

    return [parse_line(line) for line in i.splitlines()]
