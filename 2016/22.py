from utils import *

Node = namedtuple('Node', ['position', 'size', 'used', 'avail'])

size_re = re.compile(r'(\d+)T')

@check(901)
def part1(nodes):
    viable_pairs = []
    node_combinations = permutations(nodes, 2)

    for pair in node_combinations:
        node_a, node_b = pair
        if node_a.used == 0: continue

        if node_a.used <= node_b.avail:
            viable_pairs.append(pair)

    return len(viable_pairs)

State = namedtuple('State', ['data_position', 'empty_position'])

@check(238)
def part2(nodes):

    def heuristic(state):
        (data_x, data_y) = state.data_position
        return abs(data_x) + abs(data_y)

    node_dict = { node.position: node for node in nodes }

    max_x = max(x for (x, y) in node_dict.keys())
    data_pos = (max_x, 0)

    empty_pos = list(node.position for node in nodes if node.used == 0)[0]

    path = astar_search(State(data_pos, empty_pos), heuristic, partial(next_states, node_dict))

    return len(path) - 1

def next_states(nodes, state):
    states = []

    for adjacent_node_position in neighbors4(state.empty_position):
        if adjacent_node_position not in nodes: continue

        adjacent_node = nodes[adjacent_node_position]
        empty_node = nodes[state.empty_position]

        if adjacent_node.used > empty_node.size: continue

        new_state = State(
            data_position = (
                state.empty_position if state.data_position == adjacent_node_position
                else state.data_position),
            empty_position = adjacent_node_position
        )

        states.append(new_state)

    return states

def transform_input(challenge_input):
    lines = iter(challenge_input.splitlines())
    lines = (l.strip().split() for l in lines)

    # Skip the first two lines
    next(lines)
    next(lines)

    def get_size(size_str):
        return int(size_re.search(size_str).groups()[0])

    def get_node(line):
        (node_name, size, used, avail, percent) = line
        node_pos = tuple(map(int, re.search(r'/dev/grid/node-x(\d+)-y(\d+)', node_name).groups()))
        size = get_size(size)
        used = get_size(used)
        avail = get_size(avail)
        return Node(node_pos, size, used, avail)

    return [get_node(line) for line in lines]
