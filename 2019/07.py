from utils import *

from .intcode import IntcodeVm

def run_cycling_intcode(program, permutation):
    last_output = 0

    p1, p2, p3, p4, p5 = permutation

    vms = [
        IntcodeVm(program, inputs=[p1,0]),
        IntcodeVm(program, inputs=[p2]),
        IntcodeVm(program, inputs=[p3]),
        IntcodeVm(program, inputs=[p4]),
        IntcodeVm(program, inputs=[p5]),
    ]

    for (vm1, vm2) in window(cycle(vms)):
        output = vm1.run_until_next_output()
        if output is not None:
            last_output = output
            vm2.add_input(output)

        if all(vm.halted for vm in vms):
            return last_output

def get_maximising_permutation(program, values):
    outputs = {}

    for permutation in permutations(values):
        outputs[permutation] = run_cycling_intcode(program, permutation)

    best_permutation, best_value = argmax(outputs)
    return best_value

def part1(program):
    return get_maximising_permutation(program, (0, 1, 2, 3, 4))

def part2(program):
    return get_maximising_permutation(program, (5, 6, 7, 8, 9))

def transform_input(i):
    return list(map(int, i.split(',')))
