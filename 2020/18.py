from utils import *

def evaluate(i):
    if isinstance(i, int):
        return i

    total, *rest = i
    total = evaluate(total)

    for n in chunk_list(rest, 2):
        op, val = n
        val = evaluate(val)

        if op == "+":
            total += val
        else:
            total *= val

    return total

def add_extra_brackets(expr):
    if isinstance(expr, int):
        return expr

    new = []
    idx = 0

    while idx < len(expr):
        curr = expr[idx]

        if isinstance(curr, int):
            new.append(curr)
            idx += 1
        elif isinstance(curr, list):
            new.append(add_extra_brackets(curr))
            idx += 1
        elif curr == '*':
            new.append(curr)
            idx += 1
        elif curr == "+":
            last = new.pop()
            next_item = add_extra_brackets(expr[idx + 1])
            bracketed = [last, curr, next_item]
            new.append(bracketed)
            idx += 2

    return new


@check(98621258158412)
def part1(i):
    return sum(evaluate(line) for line in i)

@check(241216538527890)
def part2(i):
    return sum(evaluate(add_extra_brackets(expr)) for expr in i)

def parse(i, idx):
    tokens = []

    while idx < len(i):
        token = i[idx]

        if is_int(token):
            tokens.append(int(token))
            idx += 1
        elif token in ('+', '*'):
            tokens.append(token)
            idx += 1
        elif token == "(":
            idx, subexpr_tokens = parse(i, idx + 1)
            tokens.append(subexpr_tokens)
        elif token == ")":
            return idx + 1, tokens

    return tokens

def transform_input(i):
    def parse_expression(expr):
        # Make it slightly easier to tokenise by allowing splitting on " "
        expr = expr.replace("(", "( ").replace(")", " )")
        return parse(expr.split(), idx=0)

    return [parse_expression(line) for line in i.splitlines()]
