import re
import itertools
from utils import lfilter

def part1(lines):
    matching_lines = lfilter(contains_abba, lines)
    matching_lines = lfilter(doesnt_contain_abba_in_square_brackets, matching_lines)
    print(len(matching_lines))

def part2(lines):
    supported_lines = lfilter(is_ssl_supported, lines)
    print(len(supported_lines))

def contains_abba(line):
    return re.search(r'(.)(?!\1)(.)\2\1', line)

def doesnt_contain_abba_in_square_brackets(line):
    return not re.search(r'\[[^\]]*(.)(?!\1)(.)\2\1[^\]]*\]', line)

def get_abas(line):
    # Note: This uses a lookahead assertion (?=...) in order to get overlapping matches
    return re.findall(r'(?=(.)(?!\1)(.)\1)', line)

def is_ssl_supported(line):
    all_segments = line.replace(']', '[').split('[')

    (out_of_brackets_segments, in_brackets_segments) = segment_list(all_segments)

    (out_of_brackets_patterns, in_brackets_patterns) = (find_all_abas(out_of_brackets_segments), find_all_abas(in_brackets_segments))

    pattern_pairs = itertools.product(out_of_brackets_patterns, in_brackets_patterns)

    matching_patterns = filter(lambda t: is_inverse(t[0], t[1]), pattern_pairs)

    return any(matching_patterns)

def segment_list(l):
    segment1 = l[0::2]
    segment2 = l[1::2]
    return (segment1, segment2)

def find_all_abas(segments):
    joined_segments = " ".join(segments)
    patterns = get_abas(joined_segments)
    return patterns

def is_inverse(t1, t2):
    return t1 == (t2[1], t2[0])

def transform_input(challenge_input):
    return challenge_input.splitlines()
