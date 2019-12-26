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
    recipes: Vec<Recipe>,
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
}

impl Factory {
    pub fn new(recipes: Vec<Recipe>) -> Factory {
        Factory{ recipes: recipes, inventory: Inventory::new(), ore_mined: 0 }
    }

    pub fn mine(&mut self, amount: isize) {
        self.ore_mined += amount;
    }

    pub fn craft(&mut self, recipe: Recipe) {
        
    }
}

fn main() {
    let reader = BufReader::new(File::open("ex.0").unwrap());

    let recipes: Vec<Recipe> = reader.lines().map(|line| {
        Recipe::parse(&line.unwrap()).unwrap()
    }).collect();

    let mut factory = Factory::new(recipes);
    factory.mine(10);

    println!("{:#?}", factory);
}