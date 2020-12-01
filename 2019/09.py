from utils import *

from .intcode import IntcodeVm

@check(4261108180)
def part1(program):
    vm = IntcodeVm(program, inputs=[1])
    vm.run_until_halted()
    return vm.outputs[-1]

@check(77944)
def part2(program):
    vm = IntcodeVm(program, inputs=[2])
    vm.run_until_halted()
    return vm.outputs[-1]

def transform_input(i):
    return list(map(int, i.split(',')))
