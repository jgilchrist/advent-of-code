import re
from utils import lmap, merge_lists
from enum import Enum
from functools import reduce
import operator

def part1(instructions):
    (value_instructions, comparison_instructions) = instructions

    bots = dict()
    outputs = dict()

    # Populate bots with initial values
    for initial_value_instruction in value_instructions:
        (bot_number, value) = initial_value_instruction
        bot = add_object(bots, outputs, 'bot', bot_number)
        bot.give_value(value)

    # Populate bots without initial values
    for comparison_instruction in comparison_instructions:
        (bot_number, out1, low, out2, high) = comparison_instruction

        add_object(bots, outputs, out1, low)
        add_object(bots, outputs, out2, high)

    # Link up bots
    for comparison_instruction in comparison_instructions:
        (bot_number, out1, low, out2, high) = comparison_instruction

        low_object = get_object(bots, outputs, out1, low)
        bots[bot_number].set_low(low_object)

        high_object = get_object(bots, outputs, out2, high)
        bots[bot_number].set_high(high_object)

    # Send all values
    all_bots = bots.values()

    while not all(bot.sent for bot in all_bots):
        for bot in all_bots:
            bot.send_values()

def part2(instructions):
    (value_instructions, comparison_instructions) = instructions

    bots = dict()
    outputs = dict()

    # Populate bots with initial values
    for initial_value_instruction in value_instructions:
        (bot_number, value) = initial_value_instruction
        bot = add_object(bots, outputs, 'bot', bot_number)
        bot.give_value(value)

    # Populate bots without initial values
    for comparison_instruction in comparison_instructions:
        (bot_number, out1, low, out2, high) = comparison_instruction

        add_object(bots, outputs, out1, low)
        add_object(bots, outputs, out2, high)

    # Link up bots
    for comparison_instruction in comparison_instructions:
        (bot_number, out1, low, out2, high) = comparison_instruction

        low_object = get_object(bots, outputs, out1, low)
        bots[bot_number].set_low(low_object)

        high_object = get_object(bots, outputs, out2, high)
        bots[bot_number].set_high(high_object)

    # Send all values
    all_bots = bots.values()

    while not all(bot.sent for bot in all_bots):
        for bot in all_bots:
            bot.send_values()

    relevant_outputs = [outputs[0], outputs[1], outputs[2]]
    numbers = lmap(lambda o: o.inputs, relevant_outputs)
    numbers = merge_lists(numbers)
    product_of_numbers = reduce(operator.mul, numbers, 1)
    print(product_of_numbers)

class Comparison(Enum):
    Low = 1,
    High = 2,

def add_object(bots, outputs, object_type, id_number):
    if object_type == 'bot':
        if id_number not in bots: bots[id_number] = Bot(id_number)
        return bots[id_number]
    else:
        if id_number not in outputs: outputs[id_number] = Output(id_number)
        return outputs[id_number]

def get_object(bots, outputs, object_type, id_number):
    if object_type == 'bot':
        return bots[id_number] if id_number in bots else None
    else:
        return outputs[id_number] if id_number in outputs else None

class Bot:
    def __init__(self, id_number):
        self.id_number = id_number

        # Bots to send high/low numbers to
        self.send_high = None
        self.send_low = None

        # Values which this bot can compare
        self.inputs = []

        # Whether this bot has sent its values
        self.sent = False

    def give_value(self, value):
        self.inputs.append(value)
        assert len(self.inputs) <= 2

    def can_send_value(self):
        if self.sent: return False
        return len(self.inputs) == 2

    def send_values(self):
        if not self.send_high or not self.send_low: return
        if not self.can_send_value(): return

        if set(self.inputs) == set([61, 17]):
            print(self.id_number)

        self.send_high.give_value(max(self.inputs))
        self.send_low.give_value(min(self.inputs))
        self.sent = True

    def set_high(self, bot):
        self.send_high = bot

    def set_low(self, bot):
        self.send_low = bot

class Output(Bot):
    pass

def transform_input(challenge_input):
    def parse_value_instruction(instruction):
        (value, bot) = re.match('value (\d+) goes to bot (\d+)', instruction).groups()
        return (int(bot), int(value))

    def parse_comparison_instruction(instruction):
        (bot, out1, low, out2, high) = re.match('bot (\d+) gives low to (\w+) (\d+) and high to (\w+) (\d+)', instruction).groups()
        return (int(bot), out1, int(low), out2, int(high))

    instructions = challenge_input.splitlines()

    value_instructions = [parse_value_instruction(i) for i in instructions if i.startswith('value')]
    comparison_instructions = [parse_comparison_instruction(i) for i in instructions if not i.startswith('value')]

    return (value_instructions, comparison_instructions)
