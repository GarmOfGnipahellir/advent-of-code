use std::{collections::HashMap, fmt::Display};

use glam::{ivec2, IVec2};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/14")));
    println!("02: {}", part02(include_str!("../inputs/14")));
}

#[derive(Debug, PartialEq)]
enum Cell {
    Sand,
    Rock,
}

#[derive(Debug)]
struct Cave {
    source: IVec2,
    map: HashMap<IVec2, Cell>,
    max: IVec2,
    floor: Option<i32>,
}

impl Cave {
    fn parse(s: &str) -> Self {
        let source = ivec2(500, 0);

        let mut map = HashMap::new();
        for line in s.lines() {
            let path = line
                .split(" -> ")
                .map(|x| x.split_once(',').unwrap())
                .map(|(x, y)| ivec2(x.parse().unwrap(), y.parse().unwrap()))
                .collect::<Vec<_>>();
            for i in 1..path.len() {
                let mut from = path[i - 1];
                let mut to = path[i];
                if to.x < from.x || to.y < from.y {
                    std::mem::swap(&mut from, &mut to);
                }
                if from.y == to.y {
                    for x in from.x..=to.x {
                        map.insert(ivec2(x, from.y), Cell::Rock);
                    }
                    continue;
                }
                if from.x == to.x {
                    for y in from.y..=to.y {
                        map.insert(ivec2(from.x, y), Cell::Rock);
                    }
                    continue;
                }
                unreachable!()
            }
        }

        let mut max = source;
        for &pos in map.keys() {
            if pos.x > max.x {
                max.x = pos.x;
            }
            if pos.y > max.y {
                max.y = pos.y;
            }
        }

        Self {
            source,
            map,
            max,
            floor: None,
        }
    }

    fn add_floor(&mut self) {
        self.max.y += 2;
        self.floor = Some(self.max.y);
    }

    fn is_blocked(&self, pos: IVec2) -> bool {
        self.map.get(&pos).is_some()
            || self
                .floor
                .and_then(|y| if pos.y == y { Some(()) } else { None })
                .is_some()
    }

    fn num_sand(&self) -> usize {
        self.map.values().fold(0, |acc, c| match c {
            Cell::Sand => acc + 1,
            Cell::Rock => acc,
        })
    }

    fn drop_sand(&mut self) -> Option<IVec2> {
        let mut sand = self.source;
        loop {
            if sand.y > self.max.y {
                return None;
            }

            if !self.is_blocked(sand + IVec2::Y) {
                sand += IVec2::Y;
                continue;
            }

            if !self.is_blocked(sand + ivec2(-1, 1)) {
                sand += ivec2(-1, 1);
                continue;
            }

            if !self.is_blocked(sand + ivec2(1, 1)) {
                sand += ivec2(1, 1);
                continue;
            }

            break;
        }
        self.map.insert(sand, Cell::Sand);
        Some(sand)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut min, mut max) = (self.source, self.source);
        if let Some(y) = self.floor {
            max.y = y;
        }
        for &pos in self.map.keys() {
            if pos.x < min.x {
                min.x = pos.x;
            }
            if pos.y < min.y {
                min.y = pos.y;
            }
            if pos.x > max.x {
                max.x = pos.x;
            }
            if pos.y > max.y {
                max.y = pos.y;
            }
        }
        min -= IVec2::X;
        max += IVec2::X;
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = ivec2(x, y);
                match self.map.get(&pos) {
                    Some(cell) => match cell {
                        Cell::Sand => write!(f, "O")?,
                        Cell::Rock => write!(f, "#")?,
                    },
                    None => {
                        if self
                            .floor
                            .and_then(|y| if pos.y == y { Some(()) } else { None })
                            .is_some()
                        {
                            write!(f, "#")?;
                            continue;
                        }
                        if pos == self.source {
                            write!(f, "+")?;
                            continue;
                        }
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part01(input: &str) -> usize {
    let mut cave = Cave::parse(input);
    while cave.drop_sand().is_some() {}
    cave.num_sand()
}

fn part02(input: &str) -> usize {
    let mut cave = Cave::parse(input);
    cave.add_floor();
    while cave.map.get(&cave.source).is_none() {
        cave.drop_sand();
    }
    cave.num_sand()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        println!("{}", Cave::parse(EXAMPLE));
    }

    #[test]
    fn test_drop_sand() {
        let mut cave = Cave::parse(EXAMPLE);
        cave.drop_sand();
        println!("{cave}");
        cave.drop_sand();
        println!("{cave}");
        for _ in 0..3 {
            cave.drop_sand();
        }
        println!("{cave}");
        for _ in 0..17 {
            cave.drop_sand();
        }
        println!("{cave}");
        for _ in 0..2 {
            cave.drop_sand();
        }
        println!("{cave}");
        cave.drop_sand();
        println!("{cave}");
        assert_eq!(cave.num_sand(), 24);
    }

    #[test]
    fn test_drop_sand_floor() {
        let mut cave = Cave::parse(EXAMPLE);
        cave.add_floor();
        for _ in 0..93 {
            cave.drop_sand();
        }
        println!("{cave}");
        assert_eq!(cave.map.get(&cave.source), Some(&Cell::Sand));
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 24);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 93);
    }

    const EXAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
}
