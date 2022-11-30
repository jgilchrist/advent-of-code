from utils import *

def increment_letter(letter):
    index_into_alphabet = ord(letter) - 97
    incremented_and_wrapped = (index_into_alphabet + 1) % 26
    new_letter = chr(incremented_and_wrapped + 97)
    wrapped = incremented_and_wrapped == 0
    return (new_letter, wrapped)

def increment_password(password):
    password = list(password)
    carry = 1
    idx = len(password) - 1

    while carry:
        current_letter = password[idx]
        new_letter, carry = increment_letter(current_letter)
        password[idx] = new_letter
        idx -= 1

    return password

def has_pairs(password):
    chars = list(password) + [None]
    result = []
    count = 1

    for last_char, current_char in window(chars, 2):
        if current_char == last_char:
            count += 1
        else:
            if count > 1:
                result += [last_char]
            count = 1

    return len(result) >= 2

def is_valid_password(password):
    def has_incrementing_character_run():
        for (a, b, c) in window(password, 3):
            if ord(a) == ord(b) - 1 == ord(c) - 2:
                return True

        return False

    def contains_invalid_chars():
        return 'i' in password or 'o' in password or 'l' in password

    return has_incrementing_character_run() and not contains_invalid_chars() and has_pairs(password)

def next_valid_password(password):
    while not is_valid_password(password):
        password = increment_password(password)

    return "".join(password)


@check("cqjxxyzz")
def part1(password):
    return next_valid_password(password)

@check("cqkaabcc")
def part2(password):
    new_password = part1(password)
    return next_valid_password(increment_password(new_password))

def test():
    assert increment_letter('a') == ('b', 0)
    assert increment_letter('z') == ('a', 1)

    assert "".join(increment_password('aaa')) == 'aab'
    assert "".join(increment_password('aaz')) == 'aba'
    assert "".join(increment_password('azzz')) == 'baaa'

    assert has_pairs('aabcdefgg')
    assert not has_pairs('aaaaabcdefg')

    assert not is_valid_password('hijklmmn')
    assert not is_valid_password('abbceffg')
    assert not is_valid_password('abbcegjk')
    assert is_valid_password('ghjaabcc')
