from utils import *

import itertools
from collections import Counter

def part1(i):
    words_with_letter_counts = [(word, Counter(word).values()) for word in i]

    get_words_with_n_repeats = lambda words, n: [w for (w, counts) in words if n in counts]

    words_with_two_repeated_letters = get_words_with_n_repeats(words_with_letter_counts, 2)
    words_with_three_repeated_letters = get_words_with_n_repeats(words_with_letter_counts, 3)

    return len(words_with_two_repeated_letters) * len(words_with_three_repeated_letters)

def part2(i):
    words_vs = itertools.product(i, i)

    def mismatched_letters(w1, w2):
        return [(l1, l2) for (l1, l2) in zip(w1, w2) if l1 != l2]

    all_mismatched_letters = [((w1, w2), mismatched_letters(w1, w2)) for (w1, w2) in words_vs]

    words_with_one_mismatched_letter = [
        ((w1, w2), mismatched)
        for ((w1, w2), mismatched) in all_mismatched_letters
        if len(mismatched) == 1
    ]

    (candidate_w1, candidate_w2), _ = words_with_one_mismatched_letter[0]

    common_letters = [l1 for (l1, l2) in zip(candidate_w1, candidate_w2) if l1 == l2]

    return concat(common_letters)

def transform_input(i):
    return i.splitlines()
