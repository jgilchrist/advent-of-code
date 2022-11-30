from utils import *

def rule_has_bag(rule, bag):
    (before, after) = rule
    return any(color == bag for (n, color) in after)

@check(121)
def part1(rules):
    have = {'shiny gold'}
    last_have = set()

    while have != last_have:
        last_have = set(have)

        for bag in have:
            rules_matching = [rule for rule in rules.items() if rule_has_bag(rule, bag)]
            new_bags = set(rule[0] for rule in rules_matching)
            have = have.union(new_bags)

    # We had shiny gold to start with, so we don't count that
    return len(have) - 1

def bags_inside(rules, bag):
    nbags = 0
    result_of_rule = rules[bag]

    if not result_of_rule:
        return nbags
    else:
        for (new_amount, new_bag) in result_of_rule:
            nbags += new_amount + new_amount * bags_inside(rules, new_bag)

    return nbags

@check(3805)
def part2(rules):
    return bags_inside(rules, 'shiny gold')

def transform_input(i):
    def trim_last_word(s):
        return " ".join(s.split(" ")[:-1])

    def parse_bag(s):
        number, *color_components = s.split()
        return (int(number), " ".join(color_components))


    def parse_rule(rule):
        before, after = rule.split(" contain ")

        before = before[:-5]

        if 'no other' in after:
            after = []
        else:
            after = after.split(", ")
            after = [trim_last_word(s) for s in after]
            after = [parse_bag(s) for s in after]

        return before, after

    rules = i.splitlines()
    rules = [parse_rule(rule) for rule in rules]
    return dict(rules)
