from utils import *

def build_regex(n, rules):
    rule_result = rules[n]

    if len(rule_result) == 1 and not is_int(rule_result[0][0]):
        return rule_result[0][0]
    else:
        options = "(" + "|".join([
            "".join([build_regex(n, rules) for n in option])
            for option in rule_result
        ]) + ")"

        return options

        print(n, options)

def number_of_messages_matching(rules, messages):
    regex = "^" + build_regex(0, rules) + "$"
    matches = [m for m in messages if re.match(regex, m)]
    return len(matches)


@check(168)
def part1(i):
    rules, messages = i
    return number_of_messages_matching(rules, messages)

@check(277)
def part2(i):
    rules, messages = i

    # Patch rules
    # Hack: by inspection, rule 8 essentially allows you to repeat rule 42 an artibrary number of times
    rules[8] = [tuple(itertools.repeat(42, n)) for n in range(1, 10)]

    # Hack: by inspection, rule 11 allows for n*42 immediately followed by n*31
    rules[11] = [[*itertools.repeat(42, n), *itertools.repeat(31, n)] for n in range(1, 10)]

    return number_of_messages_matching(rules, messages)

def transform_input(i):
    def get_end(end):
        end = end.strip()

        if not is_int(end.replace(" ", "").replace("|", "")):
            return [(end[1],)]
        else:
            end = end.split("|")
            return [tuple(get_all_positive_numbers(e)) for e in end]

    def parse_rule(line):
        start, end = line.split(":")
        start = int(start)
        end = get_end(end)
        return start, end

    def parse_rules(lines):
        rules = {}

        for line in lines:
            start, end_options = parse_rule(line)
            rules[start] = end_options

        return rules

    rules, messages = i.split("\n\n")
    rules = parse_rules(rules.splitlines())
    messages = messages.splitlines()

    return rules, messages
