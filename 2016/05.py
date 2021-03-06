from utils import *

@check("c6697b55")
@slow
def part1(door_id):
    code = ""

    for hash_string in all_hashes(door_id):
        if hash_string.startswith('00000'):
            code_char = hash_string[5]
            code += code_char

            print(hash_string)
            print(code)

            if len(code) == 8:
                return code

@check("8c35d1ab")
@slow
def part2(door_id):
    code = [None] * 8

    for hash_string in all_hashes(door_id):
        if hash_string.startswith('00000'):
            position = hash_string[5]
            code_char = hash_string[6]

            position_index = int(position, base=16)

            if not position_index < 8:
                continue

            if code[position_index] is not None:
                continue

            code[position_index] = code_char

            print(hash_string)
            print(concat(i if i is not None else "*" for i in code))

            if all(i is not None for i in code):
                return concat(code)


def all_hashes(door_id):
    natural_numbers_as_strings = map(str, count())

    door_id_zipped_with_natural_numbers = zip(repeat(door_id), natural_numbers_as_strings)

    # An infinite generator of all strings of the form doorid0, doorid1, etc.
    input_strings = map(concat, door_id_zipped_with_natural_numbers)

    # An infinite generator of all hashed inputs
    all_hashed_strings = map(hash_utf, input_strings)

    return all_hashed_strings
