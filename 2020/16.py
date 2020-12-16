from utils import *

def is_in_range(rng, value):
    start, end = rng
    return start <= value <= end

def is_valid_value_for_field(field, value):
    name, ranges = field
    return any(is_in_range(r, value) for r in ranges)

def is_valid_value_for_any_field(fields, value):
    return any(is_valid_value_for_field(f, value) for f in fields)

def invalid_values_in_ticket(fields, ticket):
    return [value for value in ticket if not is_valid_value_for_any_field(fields, value)]

def is_valid_ticket(fields, ticket):
    return not invalid_values_in_ticket(fields, ticket)

def get_candidate_fields(fields, values):
    def is_candidate_field(field, values):
        return all(is_valid_value_for_field(field, value) for value in values)

    return [f for f in fields if is_candidate_field(f, values)]

@check(21071)
def part1(i):
    fields, my_ticket, nearby_tickets = i

    invalid_values = [invalid_values_in_ticket(fields, t) for t in nearby_tickets]

    return sum(flatten(invalid_values))

@check(3429967441937)
def part2(i):
    fields, my_ticket, nearby_tickets = i

    valid_tickets = [t for t in nearby_tickets if is_valid_ticket(fields, t)]
    values_per_field = list(transpose(valid_tickets))

    undecided_candidates = {
        i: get_candidate_fields(fields, values)
        for i, values in enumerate(values_per_field)
    }

    decided_candidates = {}

    while undecided_candidates:
        # First, see which fields now only have a single candidate and mark them as 'decided'
        for i, candidates in undecided_candidates.items():
            if len(candidates) == 1:
                decided_candidates[i] = candidates[0]

        # Filter out decided fields from 'undecided_candidates'
        undecided_candidates = { i: candidates for (i, candidates) in undecided_candidates.items() if i not in decided_candidates }

        # Remove decided fields as candidates from all other fields
        undecided_candidates = { i: [f for f in candidates if f not in decided_candidates.values()] for (i, candidates) in undecided_candidates.items() }

    name_to_index = { v[0]: k for (k, v) in decided_candidates.items() }
    relevant_indices = [index for (name, index) in name_to_index.items() if name.startswith('departure')]
    relevant_values = [my_ticket[index] for index in relevant_indices]
    return math.prod(relevant_values)

def transform_input(i):
    def parse_range(r):
        start, end = get_all_positive_numbers(r)
        return start, end

    def parse_field(field):
        name, ranges = field.split(':')
        ranges = ranges.split('or')
        ranges = [parse_range(r) for r in ranges]
        return (name, ranges)

    def parse_fields(fields):
        return [parse_field(field) for field in fields.splitlines()]

    def parse_my_ticket(my_ticket):
        header, ticket = my_ticket.splitlines()
        return list(get_all_positive_numbers(ticket))

    def parse_nearby_tickets(nearby_tickets):
        lines = nearby_tickets.splitlines()
        # Discard header
        lines = lines[1:]
        return [list(get_all_positive_numbers(line)) for line in lines]

    fields, my_ticket, nearby_tickets = i.split('\n\n')

    fields = parse_fields(fields)
    my_ticket = parse_my_ticket(my_ticket)
    nearby_tickets = parse_nearby_tickets(nearby_tickets)

    return fields, my_ticket, nearby_tickets
