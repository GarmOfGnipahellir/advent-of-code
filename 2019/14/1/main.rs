//! ```cargo
//! [dependencies]
//! ```

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Recipe {
    input: HashMap<String, isize>,
    output: HashMap<String, isize>,
}

#[derive(Debug)]
struct Inventory {
    content: HashMap<String, isize>,
}

#[derive(Debug)]
struct Factory {
    inventory: Inventory,
    ore_mined: isize,
}

impl Recipe {
    pub fn new(
        input: HashMap<String, isize>, 
        output: HashMap<String, isize>
    ) -> Recipe {
        Recipe{ input: input, output: output }
    }

    pub fn parse(text: &str) -> Result<Recipe, ()> {
        println!("{}", text);
    
        let mut iter = text.split(" => ").map(|part| {
            let mut result = HashMap::new();

            for item in part.split(", ") {
                let mut iter = item.split(" ");
                let amount = iter.next().unwrap();
                let name = iter.next().unwrap();
                result.insert(name.to_string(), amount.parse().unwrap());
            }

            result
        });

        let input = iter.next().unwrap();
        let output = iter.next().unwrap();
    
        Ok(Recipe::new(input, output))
    }
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory{ content: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, amount: isize) {
        if let Some(value) = self.content.get_mut(name) {
            *value += amount;
        } else {
            self.content.insert(name.to_string(), amount);
        }
    }

    pub fn remove(&mut self, name: &str, amount: isize) -> Result<(), isize> {
        if let Some(value) = self.content.get_mut(name) {
            if *value > amount {
                return Err(amount);
            } else {
                *value -= amount;
            }
        } else {
            return Err(amount);
        }
        Ok(())
    }
}

impl Factory {
    pub fn new() -> Factory {
        Factory{ inventory: Inventory::new(), ore_mined: 0 }
    }

    pub fn mine(&mut self, amount: isize) {
        self.ore_mined += amount;
        self.inventory.add("ORE", amount)
    }

    pub fn craft(&mut self, recipe: &Recipe) -> Result<(), HashMap<String, isize>> {
        let mut failed: HashMap<String, isize> = HashMap::new();
        for (name, amount) in &recipe.input {
            if let Err(diff) = self.inventory.remove(&name, *amount) {
                failed.insert(name.to_string(), diff);
            }
        }

        if failed.len() > 0 {
            return Err(failed);
        }
        
        for (name, amount) in &recipe.output {
            self.inventory.add(&name, *amount);
        }
        Ok(())
    }

    pub fn produce(&mut self, name: &str, amount: isize, recipes: &Vec<Recipe>) {
        if name == "ORE" {
            println!("Mining {} {}", amount, name);
            self.mine(amount);
        } else if let Ok(recipe) = get_recipe(recipes, name) {
            if let Err(failed) = self.craft(recipe) {
                println!("Tried making {} {} need {:#?}", amount, name, failed);
                for (name, diff) in &failed {
                    println!("Fixing {}", name);
                    self.produce(&name, *diff, recipes);
                }
                println!("Trying again...");
                self.craft(recipe);
            } else if let Some(value) = self.inventory.content.get(&name.to_string()) {
                if value > &amount {
                    self.produce(&name.to_string(), amount, recipes);
                    println!("Not enough {} {}", amount, name);
                } else {
                    println!("{} <= {}", value, &amount);
                }
            }
        } else {
            panic!("Can't produce {}", name);
        }
    }
}

fn get_recipe<'a>(recipes: &'a Vec<Recipe>, name: &str) -> Result<&'a Recipe, ()> {
    for recipe in recipes {
        if recipe.output.contains_key(&name.to_string()) {
            return Ok(recipe);
        }
    }
    Err(())
}

fn main() {
    let reader = BufReader::new(File::open("ex.0").unwrap());

    let recipes: Vec<Recipe> = reader.lines().map(|line| {
        Recipe::parse(&line.unwrap()).unwrap()
    }).collect();

    let mut factory = Factory::new();
    factory.produce("FUEL", 1, &recipes);

    println!("{:#?}", factory);
}