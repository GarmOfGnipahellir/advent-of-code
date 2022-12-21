use std::collections::{HashSet, VecDeque};

use glam::IVec3;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

const INPUT: &str = include_str!("../inputs/18");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

const ALL_AXES: [IVec3; 6] = [
    IVec3::X,
    IVec3::Y,
    IVec3::Z,
    IVec3::NEG_X,
    IVec3::NEG_Y,
    IVec3::NEG_Z,
];

#[derive(Debug, PartialEq)]
struct Map {
    cubes: HashSet<IVec3>,
}

impl Map {
    fn new(cubes: &[IVec3]) -> Self {
        Self {
            cubes: HashSet::from_iter(cubes.iter().cloned()),
        }
    }

    fn parse(s: &str) -> Self {
        let cubes = s
            .lines()
            .map(|l| {
                IVec3::from_slice(&l.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();
        Self::new(&cubes)
    }

    fn is_occupied(&self, p: IVec3) -> bool {
        self.cubes.contains(&p)
    }

    fn open_faces(&self) -> Vec<IVec3> {
        let mut res = Vec::new();
        for &p in &self.cubes {
            for d in ALL_AXES {
                if !self.is_occupied(p + d) {
                    res.push(p + d);
                }
            }
        }
        res
    }

    fn count_open_faces(&self) -> usize {
        self.open_faces().len()
    }

    fn bounds(&self) -> (IVec3, IVec3) {
        self.cubes.iter().fold(
            (IVec3::ONE * i32::MAX, IVec3::ONE * i32::MIN),
            |(min, max), &p| (min.min(p), max.max(p)),
        )
    }

    fn is_contained(&self, p: IVec3) -> bool {
        let goal = self.bounds().0 - IVec3::ONE;
        let mut frontier = VecDeque::new();
        frontier.push_back(p);
        let mut reached = HashSet::new();
        reached.insert(p);
        while let Some(current) = frontier.pop_front() {
            if current == goal {
                return false;
            }

            for d in ALL_AXES {
                let next = current + d;

                if !self.is_occupied(next) && reached.insert(next) {
                    frontier.push_back(next);
                }
            }
        }
        true
    }

    fn count_exterior_faces(&self) -> usize {
        self.open_faces()
            .par_iter()
            .map(|p| (!self.is_contained(*p)) as usize)
            .sum()
    }
}

fn part01(input: &str) -> usize {
    Map::parse(input).count_open_faces()
}

fn part02(input: &str) -> usize {
    Map::parse(input).count_exterior_faces()
}

#[cfg(test)]
mod tests {
    use glam::ivec3;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Map::parse("1,1,1\n2,1,1"),
            Map::new(&[ivec3(1, 1, 1), ivec3(2, 1, 1)])
        );
    }

    #[test]
    fn test_count_open_faces() {
        assert_eq!(Map::parse("1,1,1\n2,1,1").count_open_faces(), 10);
    }

    #[test]
    fn test_is_contained() {
        assert!(Map::parse(EXAMPLE).is_contained(ivec3(2, 2, 5)));
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 64);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 58);
    }

    const EXAMPLE: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
}
