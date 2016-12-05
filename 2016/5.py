import hashlib

def part1(door_id):
    code = ""

    for hash_string in all_hashed_strings(door_id):
        if hash_string.startswith('00000'):
            code_char = hash_string[5]
            code += code_char

            print(hash_string)
            print(code)

            if len(code) == 8:
                return

def part2(door_id):
    code = [None] * 8

    for hash_string in all_hashed_strings(door_id):
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
            print("".join(i if i is not None else "*" for i in code))

            if all(i is not None for i in code):
                return


def all_hashed_strings(door_id):
    # An infinite generator of all strings of the form doorid0, doorid1, etc.
    all_hash_inputs = hash_inputs(door_id)

    # An infinite generator of all hashed inputs
    all_hashed_strings = map(hash_utf, all_hash_inputs)

    return all_hashed_strings

def hash_inputs(door_id):
    import itertools

    natural_numbers_as_strings = map(str, itertools.count())

    # This is a trick to zip door_id with all elements in itertools.count()
    # This produces an infinite iterable of door_id0, door_id1, door_id2 ...
    door_id_zipped_with_natural_numbers = itertools.zip_longest('', natural_numbers_as_strings, fillvalue=door_id)

    concatenated_hash_strings = map(lambda x: "".join(x), door_id_zipped_with_natural_numbers)

    return concatenated_hash_strings

def hash_utf(string):
    m = hashlib.md5()
    m.update(string.encode('utf-8'))
    return m.hexdigest()
