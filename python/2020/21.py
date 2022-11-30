from utils import *

def solve(i):
    seen_with = defaultdict(list)
    solved = {}

    for ingredients, allergens in i:
        for allergen in allergens:
            seen_with[allergen].append(set(ingredients))

    last_solved = None

    while solved != last_solved:
        last_solved = dict(solved)

        for allergen, ingredients in seen_with.items():
            possible_ingredients = set.intersection(*ingredients)

            if len(possible_ingredients) == 1:
                solved[possible_ingredients.pop()] = allergen

        seen_with = { allergen: ingredients for allergen, ingredients in seen_with.items() if allergen not in solved.values() }
        seen_with = { allergen: [set(i for i in ingredient_list if i not in solved) for ingredient_list in ingredients] for allergen, ingredients in seen_with.items() }

    return solved

@check(1679)
def part1(i):
    all_ingredients = set(flatten([ingredients for ingredients, _ in i]))
    solved = solve(i)

    ingredients_with_allergens = set(solved.keys())
    ingredients_without_allergens = all_ingredients - ingredients_with_allergens

    seen_nonallergen_count = sum(len(ingredients_without_allergens.intersection(ingredients)) for ingredients, _ in i)
    return seen_nonallergen_count

@check("lmxt,rggkbpj,mxf,gpxmf,nmtzlj,dlkxsxg,fvqg,dxzq")
def part2(i):
    solved = solve(i)
    sorted_solved = sorted(solved.items(), key=operator.itemgetter(1))
    dangerous_ingredient_list = ",".join(ingredient for ingredient, allergen in sorted_solved)
    return dangerous_ingredient_list

def transform_input(i):
    def parse_line(line):
        if "(contains" in line:
            ingredients, allergens = line.replace(")", "").split(" (contains ")
            allergens = allergens.split(", ")
        else:
            ingredients, allergens = line, []

        ingredients = ingredients.split(" ")
        return ingredients, allergens

    return [parse_line(line) for line in i.splitlines()]
