from utils import *

def part1(rooms):
    result = sum(room.sector_id for room in rooms if is_valid_room(room))
    return result

def part2(rooms):
    def is_north_pole_room(room):
        return "northpole" in unencrypt_name(room)

    north_pole_room = list(filter(is_north_pole_room, rooms))[0]
    return north_pole_room.sector_id


def is_valid_room(room):
    letters = room.letters.replace("-", "")

    # [('a', 5), ('b', 3), ('y', 1), ('z', 1), ('x', 1), ('n', 1)]
    most_common_letters_with_counts = Counter(letters).most_common()

    # [('a', 5), ('b', 3), ('n', 1), ('x', 1), ('y', 1), ('z', 1)]
    sorted_by_count_then_alphabetic = sorted(most_common_letters_with_counts, key = lambda x: (-x[1], x[0]))

    # "abnxy"
    five_most_common_letters = concat(letter for (letter, count) in sorted_by_count_then_alphabetic[:5])

    return five_most_common_letters == room.checksum

def unencrypt_name(room):
    shifted_name = [
        shift(c, room.sector_id) if c.isalpha() else " "
        for c in room.letters
    ]

    shifted_name = concat(shifted_name)
    return shifted_name

def shift(char, amount):
    alphabet_index = ord(char) - 97
    shifted = (alphabet_index + amount) % 26
    new_char = chr(shifted + 97)
    return new_char


Room = namedtuple('Room', ['letters', 'sector_id', 'checksum'])

def transform_input(challenge_input):
    regex = re.compile("^([a-z-]+)-([\d]+)\[([a-z]+)\]$")

    def parse_room(line):
        match = re.search(regex, line)
        (letters, sector_id, checksum) = match.groups()
        return Room(letters, int(sector_id), checksum)

    lines = challenge_input.splitlines()
    return map(parse_room, lines)
