from utils import *

class State:
    def __init__(self, elevator, objects):
        self.elevator = elevator

        # Keeping the list of objects sorted ensures that states which
        # are equivalent will always be equal. This hugely cuts down
        # the runtime of the program.
        self.objects = tuple(sorted(objects))

    def __eq__(self, other):
        return self.elevator == other.elevator and self.objects == other.objects

    def __hash__(self):
        return hash((self.elevator, self.objects))

    def __lt__(self, other):
        return self.elevator < other.elevator and self.objects < other.objects

    def __repr__(self):
        return f"Floor {self.elevator}: {self.objects}"

def generator(element): return element[0]
def microchip(element): return element[1]

def heuristic(state):
    distances = 0
    for generator, microchip in state.objects:
        distances += 3 - generator
        distances += 3 - microchip

    return distances

def pair_is_inverse(p1, p2):
    (p2x, p2y) = p2
    return p1 == (p2y, p2x)

def part1(_):
    initial_state = State(elevator=0, objects=((0, 0), (1, 2), (1, 2), (1, 2), (1, 2)))
    path_to_goal = astar_search(initial_state, heuristic, get_next_states)
    return len(path_to_goal) - 1

def part2(a):
    initial_state = State(elevator=0, objects=((0, 0), (0, 0), (0, 0), (1, 2), (1, 2), (1, 2), (1, 2)))
    path_to_goal = astar_search(initial_state, heuristic, get_next_states)
    return len(path_to_goal) - 1


def get_next_states(state):
    current_floor = state.elevator
    objects = state.objects

    # List of deltas (+1/-1) which the elevator can travel
    available_moves = []
    if current_floor < 3: available_moves.append(+1)
    if current_floor > 0: available_moves.append(-1)

    # Lists of all generators/microchips on the current floor
    objects_on_current_floor = lambda by_fn: [o for o in objects if by_fn(o) == current_floor]
    generators_on_current_floor = objects_on_current_floor(generator)
    microchips_on_current_floor = objects_on_current_floor(microchip)

    # Combined two lists of all states reachable from moving either up or down
    return flatten([
        get_states_in_direction(state, generators_on_current_floor, microchips_on_current_floor, delta)
        for delta in available_moves
    ])

def get_states_in_direction(state, generators, microchips, delta):
    # A list of pairs of the form (object, delta)
    # e.g. If the elevator is on floor 0 and floor 0 contains a generator
    # and a microchip, possible_moves is:
    #   [
    #       ((0, 0), (1, 0)),   # < The generator can be moved up one floor
    #       ((0, 0), (0, 1)),   # < The microchip can be moved up one floor
    #   ]
    #
    possible_moves = list(chain(
        [(o, (delta, 0)) for o in generators],
        [(o, (0, delta)) for o in microchips],
    ))

    # All possible combinations of moves
    # Lists of length 1 and 2 represent the ability to carry either 1 or 2
    # items in the elevator
    possible_move_combinations = chain(
        combinations(possible_moves, 1),
        combinations(possible_moves, 2),
    )

    # If moves are to be applied to the same object (i.e. deltas are
    # (1, 0) and (0, 1)), merge these into a single move - (1, 1)
    #
    # Any moves to equivalent objects (e.g. two moves to two paired microchips
    # and generators) are also equivalent, so the result is a set to ensure
    # we don't consider more moves than are necessary.
    all_possible_moves = set(merge_moves_to_same_object(possible_move_combinations))

    # Apply deltas, moving from a list of the form (objects_before, delta) to a list
    # of the form (objects_before, objects_after).
    all_possible_moves = [
        [(before, tuple_add(before, delta)) for (before, delta) in move_components]
        for move_components in all_possible_moves
    ]

    # For every possible move in the 'delta' direction, apply this move to the current
    # state, yielding a new state with objects on new floors.
    all_possible_states = [
        make_move(state, move, delta)
        for move in all_possible_moves
    ]

    # Filter out states in which we fry any microchips
    all_possible_states = filter(is_valid_state, all_possible_states)

    return (State(*s) for s in all_possible_states)

def make_move(old_state, move, delta):
    old_elevator = old_state.elevator
    old_objects = old_state.objects

    elevator = old_elevator + delta

    objects = list(old_objects)

    for moved_object in move:
        (before, after) = moved_object
        objects.remove(before)
        objects.append(after)

    new_state = (elevator, objects)
    return (elevator, tuple(objects))

def is_valid_state(state):
    (elevator, objects) = state

    for (generator, microchip) in objects:
        # If a generator and microchip are paired, they're safe
        if generator == microchip:
            continue

        # If any other generator is on the same floor as an unpaired
        # microchip, the microchip is fried.
        for (other_generator, _) in objects:
            if other_generator == microchip:
                return False

    return True

def merge_moves_to_same_object(moves):
    moves_after_merging = []

    # Iterate through each possible move
    for move in moves:
        # List the items which will be taken in that move
        move_actions = list(move)

        # Get pairs of items
        move_pairs = list(combinations(move_actions, r=2))

        # For all pairs of moves
        for move_pair in move_pairs:
            (move1, move2) = move_pair
            (before1, delta1) = move1
            (before2, delta2) = move2

            # If they are the same item, merge them
            if before1 == before2 and pair_is_inverse(delta1, delta2):
                move_actions.remove(move1)
                move_actions.remove(move2)
                move_actions.append((before1, tuple_add(delta1, delta2)))

        moves_after_merging.append(tuple(move_actions))

    return moves_after_merging
