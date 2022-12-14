use std::collections::{HashMap, HashSet, VecDeque};

use glam::{ivec2, IVec2};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/12")));
    println!("02: {}", part02(include_str!("../inputs/12")));
}

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    elevations: Vec<i32>,
}

impl Map {
    fn elevation(&self, pos: IVec2) -> i32 {
        self.elevations[(pos.x + pos.y * self.width) as usize]
    }
}

fn part01(input: &str) -> usize {
    let (start, end, map) = {
        let mut start = IVec2::ZERO;
        let mut end = IVec2::ZERO;
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let mut elevations = Vec::new();

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                elevations.push(match c {
                    'S' => {
                        start = ivec2(j as i32, i as i32);
                        0
                    }
                    'E' => {
                        end = ivec2(j as i32, i as i32);
                        25
                    }
                    c => (c as u32 - 97) as i32,
                });
            }
        }

        (
            start,
            end,
            Map {
                width,
                height,
                elevations,
            },
        )
    };

    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    let mut came_from = HashMap::new();

    while let Some(current) = frontier.pop_front() {
        for next in [IVec2::NEG_X, IVec2::NEG_Y, IVec2::X, IVec2::Y].map(|v| current + v) {
            if next.x < 0 || next.y < 0 || next.x >= map.width || next.y >= map.height {
                continue;
            }

            if map.elevation(next) - map.elevation(current) > 1 {
                continue;
            }

            if !came_from.contains_key(&next) {
                came_from.insert(next, current);
                frontier.push_back(next);
            }
        }
    }

    let mut current = end;
    let mut path = Vec::new();
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }

    path.len()
}

fn part02(input: &str) -> usize {
    let (end, map) = {
        let mut end = IVec2::ZERO;
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let mut elevations = Vec::new();

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                elevations.push(match c {
                    'S' => 0,
                    'E' => {
                        end = ivec2(j as i32, i as i32);
                        25
                    }
                    c => (c as u32 - 97) as i32,
                });
            }
        }

        (
            end,
            Map {
                width,
                height,
                elevations,
            },
        )
    };

    let mut frontier = VecDeque::new();
    frontier.push_back(end);
    let mut came_from = HashMap::new();

    let mut starts = HashSet::new();
    while let Some(current) = frontier.pop_front() {
        for next in [IVec2::NEG_X, IVec2::NEG_Y, IVec2::X, IVec2::Y].map(|v| current + v) {
            if next.x < 0 || next.y < 0 || next.x >= map.width || next.y >= map.height {
                continue;
            }

            if map.elevation(current) - map.elevation(next) > 1 {
                continue;
            }

            if !came_from.contains_key(&next) {
                came_from.insert(next, current);
                frontier.push_back(next);
            }

            if map.elevation(next) == 0 {
                starts.insert(next);
            }
        }
    }

    let mut path_lens = Vec::new();
    for start in starts {
        let mut current = start;
        let mut path = Vec::new();
        while current != end {
            path.push(current);
            current = *came_from.get(&current).unwrap();
        }
        path_lens.push(path.len());
    }
    path_lens.sort();
    path_lens[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
        assert_eq!(part01(input), 31);
    }

    #[test]
    fn example02() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
        assert_eq!(part02(input), 29);
    }
}
