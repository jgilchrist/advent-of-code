from utils import *

def hash_state(deck1, deck2):
    return (tuple(deck1), tuple(deck2))

def play_game(deck1, deck2, use_recurse_rule):
    deck1 = list(deck1)
    deck2 = list(deck2)
    seen = set()

    while deck1 and deck2:

        state_hash = hash_state(deck1, deck2)
        if state_hash in seen:
            # Player 1 wins instantly
            return ([1], [])
        else:
            seen.add(state_hash)

        next_p1, next_p2 = deck1.pop(0), deck2.pop(0)

        if len(deck1) >= next_p1 and len(deck2) >= next_p2 and use_recurse_rule:
            subgame_d1, subgame_d2 = deck1[:next_p1], deck2[:next_p2]
            subgame_d1, subgame_d2 = play_game(subgame_d1, subgame_d2, use_recurse_rule)

            p1_won = bool(subgame_d1)
        else:
            p1_won = next_p1 > next_p2

        winning_deck = deck1 if p1_won else deck2
        winning_card, losing_card = (next_p1, next_p2) if p1_won else (next_p2, next_p1)

        winning_deck.append(winning_card)
        winning_deck.append(losing_card)

    return deck1, deck2

def score_deck(deck):
    return sum(d * multiplier for d, multiplier in zip(reversed(deck), itertools.count(1)))

@check(33680)
def part1(i):
    deck1, deck2 = i
    deck1, deck2 = play_game(deck1, deck2, use_recurse_rule=False)
    winning_deck = deck1 if deck1 else deck2
    return score_deck(winning_deck)

@check(33683)
def part2(i):
    deck1, deck2 = i
    deck1, deck2 = play_game(deck1, deck2, use_recurse_rule=True)
    winning_deck = deck1 if deck1 else deck2
    return score_deck(winning_deck)

def transform_input(i):
    def parse_deck(deck):
        header, *cards = deck.splitlines()
        cards = [int(card) for card in cards]
        return cards

    deck1, deck2 = i.split("\n\n")
    deck1, deck2 = parse_deck(deck1), parse_deck(deck2)
    return deck1, deck2
