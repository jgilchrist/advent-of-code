from utils import *

Reaction = namedtuple('Reaction', ['before', 'after'])
Element = namedtuple('Element', ['name', 'n'])

def amounts_equal(a, b):
    a = { k: v for k, v in a.items() if value != 0 }
    b = { k: v for k, v in b.items() if value != 0 }
    return a == b

def can_react(contents, reaction):
    before, after = reaction
    ingredient, nrequired = reaction.after
    return contents[ingredient] >= nrequired

def react(contents, reaction):
    assert can_react(contents, reaction)
    before, after = reaction
    new_contents = defaultdict(int, contents)
    new_contents[after.name] -= after.n
    if new_contents[after.name] == 0:
        new_contents.pop(after.name, None)

    for element in before:
        new_contents[element.name] += element.n

    return new_contents


# def get_ore_for(have, reactions):
#     have

#     ore_for_reaction = { 
#         reaction: get_ore_for(react(have, reaction), reactions)
#         for reaction in reactions
#     }

#     best_reaction, ore = argmin(ore_for_reaction)


def part1(i):
    have = defaultdict(int, { 'FUEL': 1 })

    # while any(k != "ORE" for k in have.keys()):
    #     have = 

def part2(i):
    pass

def transform_input(i):
    def parse_element(el):
        n, name = el.split()
        n = int(n)
        name = name.strip()
        return Element(name, n)

    def parse_line(line):
        before, after = line.split('=>')
        before_elements = list(map(parse_element, before.split(', ')))
        after_element = parse_element(after)
        return Reaction(before_elements, after_element)

    return [parse_line(line) for line in i.splitlines()]

def test():
    # have_ingredients:
    assert can_react(
        defaultdict(int, {'A': 1}),
        Reaction([], Element('A', 1))
    )

    assert not can_react(
        defaultdict(int, {'A': 0}),
        Reaction([], Element('A', 1))
    )

    print(react(
        defaultdict(int, {'A': 1}),
        Reaction([Element('B', 1)], Element('A', 1))
    ))
    assert react(
        defaultdict(int, {'A': 1}),
        Reaction([Element('B', 1)], Element('A', 1))
    ) == {'B': 1}
