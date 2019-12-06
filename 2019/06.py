from utils import *

def find_parent(relations, node):
    for parent, children in relations.items():
        if node in children:
            return parent

    # Node is the root (no parent found)
    return None

def find_all_parents(relations, node):
    parent = find_parent(relations, node)
    if parent is None: return []
    return [parent] + find_all_parents(relations, parent)

def get_distance_to(relations, node, toNode):
    parent = find_parent(relations, node)
    if parent == toNode: return 1
    if parent == None: return None

    distance_from_parent = get_distance_to(relations, parent, toNode)
    if distance_from_parent == None: return None

    return 1 + distance_from_parent

def get_distance_to_root(relations, node):
    return len(find_all_parents(relations, node))

def part1(i):
    relations = defaultdict(set)
    for a, b in i:
        relations[a].add(b)

    all_nodes = set(relations.keys()) | set(flatten(relations.values()))

    return sum(get_distance_to_root(relations, node) for node in all_nodes)

def part2(i):
    relations = defaultdict(set)
    for a, b in i:
        relations[a].add(b)

    all_nodes = set(relations.keys()) | set(flatten(relations.values()))

    parents_of_you = find_all_parents(relations, 'YOU')
    parents_of_san = find_all_parents(relations, 'SAN')

    first_parent_of_you = parents_of_you[0]
    first_parent_of_san = parents_of_san[0]

    parents_of_you_and_san = [i for i in parents_of_you if i in parents_of_san]

    closest_common_parent = parents_of_you_and_san[0]

    return get_distance_to(relations, first_parent_of_you, closest_common_parent) + get_distance_to(relations, first_parent_of_san, closest_common_parent)

def transform_input(i):
    i = i.splitlines()
    i = [line.split(')') for line in i]
    return i
