from utils import *

from .intcode import IntcodeVm

def run_patched_program(program, noun, verb):
    patched_program = program.copy()
    patched_program[1] = noun
    patched_program[2] = verb

    vm = IntcodeVm(patched_program)
    vm.run_until_halted()
    return vm.program[0]

@check(3058646)
def part1(program):
    return run_patched_program(program, noun=12, verb=2)

@check(8976)
def part2(program):
    for noun in range(100):
        for verb in range(100):
            program_result = run_patched_program(program, noun, verb)
            if program_result == 19690720:
                return 100 * noun + verb

    raise Exception("No result found")

def transform_input(i):
    return list(map(int, i.split(',')))
