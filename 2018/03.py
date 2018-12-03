from utils import *

import re
from collections import namedtuple, defaultdict

Claim = namedtuple('Claim', [
    'id',
    'from_left',
    'from_top',
    'width',
    'height',
])

def get_tiles_for_claim(claim):
    for y in range(claim.height):
        for x in range(claim.width):
            yield (claim.from_top + y, claim.from_left + x)

def part1(i):
    claim_memberships = defaultdict(set)

    for claim in i:
        tiles_for_claim = get_tiles_for_claim(claim)

        for tile in tiles_for_claim:
            claim_memberships[tile].add(claim.id)

    tiles_with_more_than_one_membership = [
        tile
        for (tile, memberships) in claim_memberships.items()
        if len(memberships) > 1
    ]

    return len(tiles_with_more_than_one_membership)

def part2(i):
    claim_memberships = defaultdict(set)

    for claim in i:
        tiles_for_claim = get_tiles_for_claim(claim)

        for tile in tiles_for_claim:
            claim_memberships[tile].add(claim.id)

    for claim in i:
        tiles_for_claim = get_tiles_for_claim(claim)

        memberships_on_same_tiles = [
            claim_memberships[tile]
            for tile in tiles_for_claim
        ]

        if all(len(memberships) == 1 for memberships in memberships_on_same_tiles):
            return claim.id

def transform_input(i):
    number_matches = [get_all_numbers(line) for line in i.splitlines()]
    claims = [Claim(*values) for values in number_matches]
    return claims
