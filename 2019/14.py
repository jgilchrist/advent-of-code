from utils import *

Reaction = namedtuple('Reaction', ['before', 'after'])
Element = namedtuple('Element', ['n', 'name'])

def part1(i):
    print(i)

def part2(i):
    pass

def transform_input(i):
    def parse_element(el):
        n, name = el.split()
        n = int(n)
        name = name.strip()
        return Element(n, name)

    def parse_line(line):
        before, after = line.split('=>')
        before_elements = list(map(parse_element, before.split(', ')))
        after_element = parse_element(after)
        return Reaction(before_elements, after_element)

    return [parse_line(line) for line in i.splitlines()]
