from utils import *

@check("gfdhebac")
def part1(instructions):
    return encode(instructions, 'abcdefgh')

@check("dhaegfbc")
def part2(instructions):
    for permutation in permutations('abcdefgh'):
        if encode(instructions, permutation) == 'fbgdceah':
            return concat(permutation)

def encode(instructions, passcode):
    functions = map(get_function, instructions)
    state = reduce(apply_fn, functions, passcode)
    return state

def swap_pos_fn(pos1, pos2, state):
    new_state = list(state)
    new_state[pos1], new_state[pos2] = new_state[pos2], new_state[pos1]
    return concat(new_state)

def swap_letter_fn(letter1, letter2, state):
    pos1, pos2 = state.index(letter1), state.index(letter2)
    new_state = list(state)
    new_state[pos1], new_state[pos2] = new_state[pos2], new_state[pos1]
    return concat(new_state)

def rotate_left_fn(steps, state):
    return rotate_right_fn(-steps, state)

def rotate_right_fn(steps, state):
    new_state = deque(state)
    new_state.rotate(steps)
    return concat(new_state)

def rotate_based_on_pos_fn(letter, state):
    steps = state.index(letter) + 1
    if steps > 4: steps += 1
    return rotate_right_fn(steps, state)

def reverse_fn(pos1, pos2, state):
    new_state = list(state)
    new_state[pos1:pos2+1] = reversed(new_state[pos1:pos2+1])
    return concat(new_state)

def move_fn(pos1, pos2, state):
    new_state = list(state)
    letter = new_state[pos1]
    new_state.remove(letter)
    new_state = concat(new_state[:pos2]) + letter + concat(new_state[pos2:])
    return new_state

def get_function(instruction):
    fns = [
        (r'swap position (\d+) with position (\d+)', swap_pos_fn, int),
        (r'swap letter (\w) with letter (\w)', swap_letter_fn, str),
        (r'rotate left (\d+) steps?', rotate_left_fn, int),
        (r'rotate right (\d+) steps?', rotate_right_fn, int),
        (r'rotate based on position of letter (\w)', rotate_based_on_pos_fn, str),
        (r'reverse positions (\d+) through (\d+)', reverse_fn, int),
        (r'move position (\d+) to position (\d+)', move_fn, int),
    ]

    for (regex, fn, datatype) in fns:
        if re.match(regex, instruction):
            groups = re.search(regex, instruction).groups()
            groups = map(datatype, groups)
            return partial(fn, *groups)

def transform_input(challenge_input):
    return challenge_input.splitlines()
