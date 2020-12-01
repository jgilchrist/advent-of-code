from utils import *

@check("10010010110011010")
def part1(seed):
    return get_data_to_fill_length(seed, 272)

@check("01010100101011100")
def part2(seed):
    return get_data_to_fill_length(seed, 35651584)


def get_data_to_fill_length(seed, desired_length):
    data = str(seed)
    while len(data) < desired_length:
        data = round(data)

    data = data[:desired_length]

    checksum = calculate_checksum(data)
    while len(checksum) % 2 == 0:
        checksum = calculate_checksum(checksum)

    return checksum

def round(a):
    b = a[::-1]
    b = b.replace('0', '*')
    b = b.replace('1', '0')
    b = b.replace('*', '1')

    return a + "0" + b

def calculate_checksum(data):
    return concat([
        "1" if a == b else "0"
        for a, b in chunk(data, n=2)
    ])

def chunk(iterable, n, fillvalue=None):
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=fillvalue)
