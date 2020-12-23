from utils import *

class Node:
    def __init__(self, value):
        self.value = value

    def set_next(self, node):
        self.next = node

def build_circular_linked_list(values):
    if not values: assert False

    nodes = []
    nodes_dict = {}

    for value in values:
        node = Node(value)
        nodes.append(node)
        nodes_dict[value] = node

    for node, next_node in window(nodes, 2):
        node.set_next(next_node)

    first_node = nodes[0]
    last_node = nodes[-1]

    last_node.set_next(first_node)

    return first_node, nodes_dict

def ll_values(head):
    current = head

    yield current.value
    current = current.next

    while current != head:
        yield current.value
        current = current.next

def run_turn(head, nodes, mod):
    held_cups_start = head.next
    held_cups_end = held_cups_start.next.next

    held_values = { held_cups_start.value, held_cups_start.next.value, held_cups_start.next.next.value }

    # Remove the held cups from the list
    head.next = held_cups_end.next
    held_cups_end.next = None

    desired_destination = head.value
    while desired_destination in held_values or desired_destination == 0 or desired_destination == head.value:
        desired_destination = mod if desired_destination == 1 else desired_destination - 1

    destination = nodes[desired_destination]

    # Reattach the held cups into the list after the destination cup
    held_cups_end.next = destination.next
    destination.next = held_cups_start

    head = head.next
    return head

@check("78569234")
def part1(nums):
    max_nums = max(nums)
    head, nodes = build_circular_linked_list(nums)

    for move in range(100):
        head = run_turn(head, nodes, max_nums)

    head = nodes[1]

    values = "".join(str(value) for value in ll_values(head))

    # Trim 1 from the start
    values = values[1:]
    return values

@check(565615814504)
def part2(nums):
    numbers_after_max_num = itertools.count(max(nums) + 1)
    extra_nums = range(len(nums) + 1, 1000000 + 1)
    nums.extend(extra_nums)
    max_nums = max(nums)

    head, nodes = build_circular_linked_list(nums)

    for move in range(10000000):
        head = run_turn(head, nodes, max_nums)

    head = nodes[1]
    return head.next.value * head.next.next.value

def transform_input(i):
    chars = list(i)
    return list(map(int, chars))
