from math import ceil, floor
from os import path
__dir__ = path.dirname(__file__)


def parse_ingredient(string):
    chems = string.split(', ')
    result = {}
    for chem in chems:
        [amount, chemical] = chem.split(' ')
        result[chemical] = int(amount)
    return result


def find_requirements(recipe, needed, recipes, requirements):
    factor = ceil(needed / recipe.get_output_amount())
    for chem, amount in recipe.input.items():
        amount = ceil(amount * factor)

        if chem in requirements:
            requirements[chem] += amount
        else:
            requirements[chem] = amount

        if chem == 'ORE':
            return

        find_requirements(recipes[chem], amount, recipes, requirements)


def find_required_ore(recipes, requirements):
    result = 0
    for chem1, amount1 in requirements.items():
        for recipe in recipes.values():
            if recipe.get_output_name() != chem1:
                continue

            produced = recipe.get_output_amount()

            for chem2, amount2 in recipe.input.items():
                if chem2 != 'ORE':
                    continue

                factor = ceil(amount1 / produced)

                result += ceil(amount2 * factor)
    return result

def recursive_requirements(recipes, requirements, history = []):
    result = {}
    for chem1, amount1 in requirements.items():
        if chem1 not in recipes:
            continue

        for recipe in recipes.values():
            if recipe.get_output_name() != chem1:
                continue

            produced = recipe.get_output_amount()

            for chem2, amount2 in recipe.input.items():
                if chem2 == 'ORE':
                    continue
                
                factor = ceil(amount1 / produced)
                amount = ceil(amount2 * factor)

                if chem2 in result:
                    result[chem2] += amount
                else:
                    result[chem2] = amount
    if result != {}:
        print(result)
        history.append(result)
        recursive_requirements(recipes, result)
        return history


class Recipe:
    def __init__(self, inp, out):
        self.input = inp
        self.output = out

    def __str__(self):
        return f'{self.input} => {self.output}'

    def __repr__(self):
        return f'Recipe({self.__str__()})'

    def get_output_name(self):
        return list(self.output.keys())[0]

    def get_output_amount(self):
        return list(self.output.values())[0]


with open(f'{__dir__}/ex.4', 'r') as f:
    lines = [line.strip() for line in f.readlines()]

# build recipe list and reverse it
recipes = {}
for line in lines:
    [inp, out] = [parse_ingredient(x.strip()) for x in line.split(' => ')]
    recipe = Recipe(inp, out)
    produced = recipe.get_output_name()
    recipes[produced] = recipe
    print(recipe)

print()
requirements = {}
find_requirements(recipes['FUEL'], 1, recipes, requirements)

print()
print('\n'.join([f'{amount} {chem}' for chem, amount in requirements.items()]))
print(find_required_ore(recipes, requirements))


print()
requirements = {'FUEL': 1}
history = recursive_requirements(recipes, requirements)
print(history)
requirements = {}
for req in history:
    for chem, amount in req.items():
        if chem in requirements:
            requirements[chem] += amount
        else:
            requirements[chem] = amount
print(requirements)
print(find_required_ore(recipes, requirements))