from utils import *

width = 50
height = 6

@check(121)
def part1(instructions):
    functions = map(get_function, instructions)
    state = reduce(lambda state, fn: fn(state), functions, set())
    return len(state)

@check("""###  #  # ###  #  #  ##  ####  ##  ####  ### #    
#  # #  # #  # #  # #  # #    #  # #      #  #    
#  # #  # #  # #  # #    ###  #  # ###    #  #    
###  #  # ###  #  # #    #    #  # #      #  #    
# #  #  # # #  #  # #  # #    #  # #      #  #    
#  #  ##  #  #  ##   ##  ####  ##  ####  ### #### 
""")
def part2(instructions):
    functions = map(get_function, instructions)
    state = reduce(lambda state, fn: fn(state), functions, set())

    out = ""

    for y in range(height):
        for x in range(width):
            out += "#" if (x, y) in state else " "

        out += "\n"

    return out


def rectangle_instruction(x, y, state):
    (xs, ys) = (range(x), range(y))
    added_points = set(product(xs, ys))
    return state | added_points

def rotate_row_instruction(row, amount, state):
    affected_lights = { l for l in state if l[1] == row }

    new_lights = {
        ((x + amount) % width, y)
        for (x, y)
        in affected_lights
    }

    return (state - affected_lights) | new_lights

def rotate_col_instruction(col, amount, state):
    affected_lights = { l for l in state if l[0] == col }

    new_lights = {
        (x, (y + amount) % height)
        for (x, y)
        in affected_lights
    }

    return (state - affected_lights) | new_lights

def get_function(instruction):
    fns = [
        ('rect (\d+)x(\d+)', rectangle_instruction),
        ('rotate row y=(\d+) by (\d+)', rotate_row_instruction),
        ('rotate column x=(\d+) by (\d+)', rotate_col_instruction),
    ]

    for (regex, fn) in fns:
        if re.match(regex, instruction):
            groups = re.search(regex, instruction).groups()
            groups = map(int, groups)
            return partial(fn, *groups)

def transform_input(challenge_input):
    instruction_strings = challenge_input.splitlines()
    return instruction_strings
