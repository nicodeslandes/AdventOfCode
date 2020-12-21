from typing import Dict, List, Set, Tuple


def part1(input: List[str]) -> int:
    foods: List[Tuple[List[str], List[str]]] = []
    for line in input:
        [ingredients, allergens] = line.split(" (contains ")
        ingredients = ingredients.split(' ')
        allergens = allergens[:-1].split(', ')
        foods.append((ingredients, allergens))

    allergens_ingredients_repo: Dict[str, Set[str]] = {}
    all_ingredients = set()
    for (ingredients, allergens) in foods:
        all_ingredients.update(ingredients)
        for a in allergens:
            allergens_ingredients = allergens_ingredients_repo.get(a)
            if allergens_ingredients is None:
                allergens_ingredients_repo[a] = set(ingredients)
            else:
                allergens_ingredients.intersection_update(ingredients)

    allergen_free_ingredients = [i for i in all_ingredients if not any(
        ai for ai in allergens_ingredients_repo.values() if i in ai)]

    return sum(1 for i in allergen_free_ingredients for (il, _) in foods if i in il)

# def part2(input: List[str]) -> int:
