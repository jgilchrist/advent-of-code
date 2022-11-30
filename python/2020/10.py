from utils import *

@check(2170)
def part1(jolts):
    adapter_chain = list(sorted(jolts))
    diffs = [y - x for x, y in window(adapter_chain, 2)]
    return diffs.count(1) * diffs.count(3)

@check(24803586664192)
def part2(jolts):
    adapter_chain = list(sorted(jolts))

    memo = {}

    def number_of_completions(chain):
        if len(chain) == 1:
            return 1

        current_adapter = chain[0]

        # If we already know the number of combinations from this point in the chain
        # then we don't need to recurse
        if current_adapter in memo:
            return memo[current_adapter]

        # We can pick any adapter within 3 of the current one
        next_adapters = [a for a in chain[1:] if a <= current_adapter + 3]

        # Build the chains that result from picking each of these adapters
        possible_next_chains = [
            [a for a in chain if not a < next_adapter]
            for next_adapter in next_adapters
        ]

        # Sum the different possibilities for each of the possible chains
        nways = sum(number_of_completions(next_chain) for next_chain in possible_next_chains)
        memo[current_adapter] = nways

        return nways

    return number_of_completions(adapter_chain)

def transform_input(i):
    i = [int(line) for line in i.splitlines()]
    device_adapter = max(i) + 3
    return [0, *i, device_adapter]
