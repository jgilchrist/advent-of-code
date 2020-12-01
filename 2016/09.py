from utils import *

# This regex takes a string and splits it up into four parts
#   * The text before the next compression marker: (.*?)
#   * The next compression marker's length: (\d+)
#   * The next compression marker's repeat count: (\d+)
#   * The text after the next compression marker: (.*)
regex = re.compile('(.*?)\((\d+)x(\d+)\)(.*)')

@check(120765)
def part1(challenge_input):
    length = 0
    remaining_content = challenge_input

    while re.match(regex, remaining_content):
        matches = re.search(regex, remaining_content).groups()
        (additional_length, remaining_content) = process_next_match(matches)
        length += additional_length

    return length

def process_next_match(matches):
    (non_compressed_string, number_of_chars, repeat, rest) = matches
    number_of_chars = int(number_of_chars)
    repeat = int(repeat)

    chars_to_decompress = rest[:number_of_chars]
    left_to_decompress = rest[number_of_chars:]

    length = len(non_compressed_string) + len(chars_to_decompress) * repeat

    return (length, left_to_decompress)

#######################################

@check(11658395076)
def part2(challenge_input):
    return get_length_recurse(challenge_input)

def get_length_recurse(string):
    length = 0
    chars = iter(string)

    get_next_number = lambda s: ''.join(takewhile(lambda c: c not in 'x)', chars))

    for char in chars:
        if char == '(':
            look_forward, repeat = map(int, [get_next_number(chars) for c in (0, 1)])
            rest = ''.join(islice(chars, look_forward))
            length += get_length_recurse(rest) * repeat
        else:
            length += 1

    return length
