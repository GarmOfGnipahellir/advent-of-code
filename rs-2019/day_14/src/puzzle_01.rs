use anyhow::{anyhow, Error};
use shared::read_string;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    string::ParseError,
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Material {
    Ore,
    Fuel,
    Chem(String),
}

impl FromStr for Material {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ORE" => Material::Ore,
            "FUEL" => Material::Fuel,
            s => Material::Chem(s.to_string()),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ingredient {
    amount: i64,
    material: Material,
}

impl FromStr for Ingredient {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let amount = iter
            .next()
            .ok_or(anyhow!("Couldn't get amount."))?
            .parse::<i64>()?;
        let material = iter
            .next()
            .ok_or(anyhow!("Couldn't get material."))?
            .parse::<Material>()?;

        Ok(Self { amount, material })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Recipe {
    inputs: Vec<Ingredient>,
    output: Ingredient,
}

impl FromStr for Recipe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" => ");
        let inputs: Vec<_> = iter
            .next()
            .ok_or(anyhow!("Couldn't get inputs."))?
            .split(", ")
            .map(|s| s.parse::<Ingredient>())
            .collect::<Result<Vec<_>, _>>()?;
        let output = iter
            .next()
            .ok_or(anyhow!("Couldn't get output."))?
            .parse::<Ingredient>()?;

        Ok(Self { inputs, output })
    }
}

#[derive(Debug)]
struct Node<'a> {
    recipe: Recipe,
    suppliers: Arc<Mutex<Vec<&'a Node<'a>>>>,
}

fn count_requirements(
    node: &Node,
    total: &mut HashMap<Material, i64>,
    current: &mut HashMap<Material, i64>,
) {
    let Node { recipe, suppliers } = node;
    total
        .entry(recipe.output.material.clone())
        .and_modify(|a| *a += recipe.output.amount)
        .or_insert(recipe.output.amount);
    current
        .entry(recipe.output.material.clone())
        .and_modify(|a| *a += recipe.output.amount)
        .or_insert(recipe.output.amount);

    for i in &recipe.inputs {
        if i.material == Material::Ore {
            total
                .entry(i.material.clone())
                .and_modify(|a| *a += i.amount)
                .or_insert(i.amount);
            current
                .entry(i.material.clone())
                .and_modify(|a| *a += i.amount)
                .or_insert(i.amount);
        }
    }

    for supplier in suppliers.lock().unwrap().iter() {
        if let Some(amount) = current.get_mut(&supplier.recipe.output.material) {
            if *amount >= supplier.recipe.output.amount {
                *amount -= supplier.recipe.output.amount;
                continue;
            }
        }
        count_requirements(supplier, total, current);
    }
}

fn main() {
    let contents = read_string("day_14/data/example_01.txt").unwrap();
    let recipes = contents
        .split('\n')
        .map(|l| l.parse::<Recipe>().unwrap())
        .map(|r| Node {
            recipe: r,
            suppliers: Arc::new(Mutex::new(vec![])),
        })
        .collect::<Vec<_>>();

    let root = recipes
        .iter()
        .find(
            |Node {
                 recipe,
                 suppliers: _,
             }| recipe.output.material == Material::Fuel,
        )
        .unwrap();

    let mut leaves = VecDeque::from(vec![root]);
    while let Some(Node { recipe, suppliers }) = leaves.pop_front() {
        for input in &recipe.inputs {
            if let Some(supplier) = recipes.iter().find(
                |Node {
                     recipe: r,
                     suppliers: _,
                 }| r.output.material == input.material,
            ) {
                let mut guard = suppliers.lock().unwrap();
                guard.push(supplier);
                leaves.push_back(supplier);
            }
        }
    }

    let mut total = HashMap::new();
    let mut current = HashMap::new();
    count_requirements(root, &mut total, &mut current);
    println!("{:#?}", total);
    println!("{:#?}", current);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_material() {
        assert_eq!(Material::Ore, "ORE".parse::<Material>().unwrap());
        assert_eq!(Material::Fuel, "FUEL".parse::<Material>().unwrap());
        assert_eq!(
            Material::Chem("A".to_string()),
            "A".parse::<Material>().unwrap()
        );
        assert_eq!(
            Material::Chem("B".to_string()),
            "B".parse::<Material>().unwrap()
        );
        assert_eq!(
            Material::Chem("AB".to_string()),
            "AB".parse::<Material>().unwrap()
        );
    }

    #[test]
    fn test_parse_ingredient() {
        assert_eq!(
            Ingredient {
                amount: 10,
                material: Material::Ore
            },
            "10 ORE".parse::<Ingredient>().unwrap()
        );
        assert_eq!(
            Ingredient {
                amount: 1,
                material: Material::Chem("A".to_string())
            },
            "1 A".parse::<Ingredient>().unwrap()
        );
        assert_eq!(
            Ingredient {
                amount: 3,
                material: Material::Fuel
            },
            "3 FUEL".parse::<Ingredient>().unwrap()
        );
    }

    #[test]
    fn test_parse_recipe() {
        assert_eq!(
            Recipe {
                inputs: vec![Ingredient {
                    amount: 10,
                    material: Material::Ore
                }],
                output: Ingredient {
                    amount: 10,
                    material: Material::Chem("A".to_string())
                }
            },
            "10 ORE => 10 A".parse::<Recipe>().unwrap()
        );
        assert_eq!(
            Recipe {
                inputs: vec![
                    Ingredient {
                        amount: 7,
                        material: Material::Chem("A".to_string())
                    },
                    Ingredient {
                        amount: 1,
                        material: Material::Chem("E".to_string())
                    }
                ],
                output: Ingredient {
                    amount: 1,
                    material: Material::Fuel
                }
            },
            "7 A, 1 E => 1 FUEL".parse::<Recipe>().unwrap()
        )
    }
}
