use std::{collections::HashSet, fmt::Display};

use glam::{ivec2, IVec2};

const INPUT: &str = include_str!("../inputs/24");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

const ALL_AXES: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[derive(Debug, Clone)]
struct State {
    time: usize,
    width: i32,
    height: i32,
    blizzards: Vec<(IVec2, IVec2)>,
    hazards: HashSet<IVec2>,
}

impl State {
    fn parse(s: &str) -> Self {
        let total = s.len() as i32;
        let height = s.lines().count() as i32;
        let blizzards = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    (c != '.' && c != '#').then(|| {
                        (
                            ivec2(x as i32, y as i32),
                            match c {
                                '>' => IVec2::X,
                                '<' => IVec2::NEG_X,
                                'v' => IVec2::Y,
                                '^' => IVec2::NEG_Y,
                                _ => panic!("can't parse '{c}' as direction"),
                            },
                        )
                    })
                })
            })
            .collect::<Vec<_>>();
        let hazards = HashSet::from_iter(blizzards.iter().map(|(p, _)| *p));
        Self {
            time: 0,
            width: total / height,
            height,
            blizzards,
            hazards,
        }
    }

    fn next(&self) -> Self {
        let blizzards = self
            .blizzards
            .iter()
            .cloned()
            .map(|(p, d)| (p + d, d))
            .map(|(p, d)| {
                if p.x < 1 {
                    (ivec2(self.width - 2, p.y), d)
                } else if p.x > self.width - 2 {
                    (ivec2(1, p.y), d)
                } else if p.y < 1 {
                    (ivec2(p.x, self.height - 2), d)
                } else if p.y > self.height - 2 {
                    (ivec2(p.x, 1), d)
                } else {
                    (p, d)
                }
            })
            .collect::<Vec<_>>();
        let hazards = HashSet::from_iter(blizzards.iter().map(|(p, _)| *p));
        Self {
            time: self.time + 1,
            width: self.width,
            height: self.height,
            blizzards,
            hazards,
        }
    }

    fn start(&self) -> IVec2 {
        ivec2(1, 0)
    }

    fn end(&self) -> IVec2 {
        ivec2(self.width - 2, self.height - 1)
    }

    fn is_blocked(&self, p: IVec2) -> bool {
        (p.x < 1 || p.x > self.width - 2 || p.y < 1 || p.y > self.height - 2)
            && !(p == self.start() || p == self.end())
    }

    fn neighbours(&self, p: IVec2) -> Vec<IVec2> {
        ALL_AXES
            .iter()
            .filter_map(|d| (!self.is_blocked(p + *d)).then_some(p + *d))
            .collect()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Minute: {}", self.time)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self
                    .blizzards
                    .iter()
                    .cloned()
                    .filter(|&(p, _)| p == ivec2(x, y))
                    .collect::<Vec<_>>();
                if v.len() == 0 {
                    if self.is_blocked(ivec2(x, y)) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                } else if v.len() == 1 {
                    write!(
                        f,
                        "{}",
                        match v[0].1 {
                            IVec2::X => ">",
                            IVec2::NEG_X => "<",
                            IVec2::Y => "v",
                            IVec2::NEG_Y => "^",
                            _ => unreachable!(),
                        }
                    )?;
                } else {
                    write!(f, "{}", v.len())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn bfs(state: &State) -> Option<Vec<State>> {
    bfs_recursive(state, state.start())
}

fn bfs_recursive(state: &State, current: IVec2) -> Option<Vec<State>> {
    println!("{state}");

    if current == state.end() {
        return Some(vec![state.clone()]);
    }

    if state.is_blocked(current) || state.hazards.contains(&current) {
        return None;
    }

    let res = state
        .neighbours(current)
        .iter()
        .map(|next| bfs_recursive(&state.next(), *next))
        .chain(std::iter::once(bfs_recursive(&state.next(), current)))
        .filter_map(|state| state)
        .flatten()
        .collect::<Vec<_>>();

    if res.is_empty() {
        return None;
    }

    Some(res)
}

fn part01(input: &str) -> i32 {
    let state = State::parse(input);
    println!("{state}");
    println!("res: {:?}", bfs(&state));
    -1
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let state = State::parse(EXAMPLE);
        assert_eq!(state.width, 8);
        assert_eq!(state.height, 6);
    }

    #[test]
    fn test_is_blocked() {
        let state = State::parse(EXAMPLE_SIMPLE);
        assert!(state.is_blocked(ivec2(0, 0)));
        assert!(!state.is_blocked(ivec2(1, 0)));
        assert!(state.is_blocked(ivec2(2, 0)));
        assert!(!state.is_blocked(ivec2(3, 3)));
        assert!(state.is_blocked(ivec2(5, 5)));
        assert!(!state.is_blocked(ivec2(6, 5)));
        assert!(state.is_blocked(ivec2(7, 5)));
    }

    #[test]
    fn test_neighbours() {
        let state = State::parse(EXAMPLE_SIMPLE);
        assert_eq!(state.neighbours(state.start()).len(), 1);
        assert_eq!(state.neighbours(state.end()).len(), 1);
        assert_eq!(state.neighbours(ivec2(1, 3)).len(), 3);
        assert_eq!(state.neighbours(ivec2(3, 3)).len(), 4);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), -1);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
    }

    const EXAMPLE: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    const EXAMPLE_SIMPLE: &str = r#"#.######
#......#
#......#
#......#
#......#
######.#"#;
}
