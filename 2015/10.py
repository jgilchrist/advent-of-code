from utils import *

def compress_number(number):
    chars = list(number) + [None]
    result = []
    count = 1

    for last_char, current_char in window(chars, 2):
        if current_char == last_char:
            count += 1
        else:
            result += [str(count), last_char]
            count = 1

    return "".join(result)


@check(252594)
def part1(i):
    for _ in range(40):
        i = compress_number(i)

    return len(i)

@check(3579328)
def part2(i):
    for _ in range(50):
        i = compress_number(i)

    return len(i)

def test():
    assert compress_number("1") == "11"
    assert compress_number("11") == "21"
    assert compress_number("21") == "1211"
    assert compress_number("1211") == "111221"
    assert compress_number("111221") == "312211"
