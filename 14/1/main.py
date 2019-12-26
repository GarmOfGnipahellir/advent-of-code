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


class Factory:
    def __init__(self):
        self.recipes = {}
        self.ore_mined = 0
        self.inventory = {}

    def add_item(self, name, amount):
        if name in self.inventory:
            self.inventory[name] += amount
        else:
            self.inventory[name] = amount

    def remove_item(self, name, amount):
        if name in self.inventory:
            if self.inventory[name] >= amount:
                self.inventory[name] -= amount
                return True
        return False

    def mine_ore(self, amount):
        self.ore_mined += amount
        self.add_item('ORE', amount)

    def craft_recipe(self, recipe):
        failed = {}
        for name, amount in recipe.input.items():
            if not self.remove_item(name, amount):
                failed[name] = amount

        if len(failed) == 0:
            self.add_item(
                recipe.get_output_name(),
                recipe.get_output_amount()
            )
        return failed

    def craft_and_mine(self, name, amount):
        if name == 'ORE':
            self.mine_ore(amount)
        elif name in self.recipes:
            failed = self.craft_recipe(self.recipes[name])
            for failed_name, failed_amount in failed.items():
                self.craft_and_mine(failed_name, failed_amount)

    def main(self):
        with open(f'{__dir__}/ex.0', 'r') as f:
            lines = [line.strip() for line in f.readlines()]

        for line in lines:
            [inp, out] = [parse_ingredient(x.strip())
                          for x in line.split(' => ')]
            recipe = Recipe(inp, out)
            produced = recipe.get_output_name()
            self.recipes[produced] = recipe
            print(produced, recipe)

        while 'FUEL' not in self.inventory:
            self.craft_and_mine('FUEL', 1)
            print(self.inventory)
            print(self.ore_mined)


Factory().main()
