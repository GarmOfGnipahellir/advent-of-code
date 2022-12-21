use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../inputs/01");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

#[derive(Debug, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    const ALL: [Self; 4] = [Self::Ore, Self::Clay, Self::Obsidian, Self::Geode];

    fn parse(s: &str) -> Self {
        match s {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            _ => panic!("can't parse {s}"),
        }
    }

    fn index(&self) -> usize {
        match self {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    recipes: [[i32; 4]; 4],
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let (a, b) = s.split_once(": ").unwrap();
        let id = a.split_once(' ').unwrap().1.parse().unwrap();
        let re = Regex::new(r"^Each (\w+) robot costs (\d+) (\w+)(?: and (\d+) (\w+))?").unwrap();
        let mut recipe_iter = b.split(". ").map(|s| {
            let caps = re.captures(s).unwrap();
            let ty = Resource::parse(caps.get(1).unwrap().as_str());
            let mut costs = Vec::<(Resource, i32)>::new();
            costs.push((
                Resource::parse(caps.get(3).unwrap().as_str()),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            ));
            if let (Some(n), Some(r)) = (caps.get(4), caps.get(5)) {
                costs.push((Resource::parse(r.as_str()), n.as_str().parse().unwrap()))
            }
            (ty, costs)
        });
        let mut recipes = [[0; 4]; 4];
        while let Some((ty, costs)) = recipe_iter.next() {
            for (r, n) in costs {
                recipes[ty.index()][r.index()] = n;
            }
        }
        Self { id, recipes }
    }

    fn recipe(&self, ty: Resource) -> [i32; 4] {
        self.recipes[ty.index()]
    }
}

#[derive(Debug)]
struct Factory {
    blueprint: Blueprint,
    inventory: [i32; 4],
    robots: [i32; 4],
}

impl Factory {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            inventory: [0; 4],
            robots: [1, 0, 0, 0],
        }
    }
}

fn part01(input: &str) -> i32 {
    for line in input.lines() {
        let factory = Factory::new(Blueprint::parse(line));
    }
    0
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Resource::parse("ore"), Resource::Ore);
        assert_eq!(Resource::parse("clay"), Resource::Clay);
        assert_eq!(Resource::parse("obsidian"), Resource::Obsidian);
        assert_eq!(Resource::parse("geode"), Resource::Geode);
        let bp = Blueprint::parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        assert_eq!(bp.id, 1);
        assert_eq!(bp.recipe(Resource::Ore), [4, 0, 0, 0]);
        assert_eq!(bp.recipe(Resource::Clay), [2, 0, 0, 0]);
        assert_eq!(bp.recipe(Resource::Obsidian), [3, 14, 0, 0]);
        assert_eq!(bp.recipe(Resource::Geode), [2, 0, 7, 0]);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 33);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
    }

    const EXAMPLE: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
}
