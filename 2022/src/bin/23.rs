use std::collections::{HashMap, HashSet};

use glam::{ivec2, IVec2};

const INPUT: &str = include_str!("../inputs/23");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

const N: IVec2 = IVec2::NEG_Y;
const S: IVec2 = IVec2::Y;
const W: IVec2 = IVec2::NEG_X;
const E: IVec2 = IVec2::X;
const NE: IVec2 = ivec2(1, -1);
const NW: IVec2 = ivec2(-1, -1);
const SE: IVec2 = ivec2(1, 1);
const SW: IVec2 = ivec2(-1, 1);

const ADJACENT: [IVec2; 8] = [N, NE, E, SE, S, SW, W, NW];

fn parse(s: &str) -> HashSet<IVec2> {
    HashSet::from_iter(s.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some(ivec2(x as i32, y as i32))
            } else {
                None
            }
        })
    }))
}

fn has_adjacent(p: IVec2, set: &HashSet<IVec2>) -> bool {
    ADJACENT.iter().any(|d| set.contains(&(p + *d)))
}

fn get_proposing(all: &HashSet<IVec2>) -> HashSet<IVec2> {
    HashSet::from_iter(all.iter().cloned().filter(|p| has_adjacent(*p, all)))
}

fn try_propose(p: IVec2, filter: &[IVec2; 3], all: &HashSet<IVec2>) -> Option<IVec2> {
    filter
        .iter()
        .all(|d| !all.contains(&(p + *d)))
        .then_some(p + filter[0])
}

fn get_proposals(
    all: &HashSet<IVec2>,
    proposing: &HashSet<IVec2>,
    filters: &[[IVec2; 3]],
) -> HashMap<IVec2, IVec2> {
    HashMap::from_iter(proposing.iter().filter_map(|p| {
        filters
            .iter()
            .find_map(|f| try_propose(*p, f, all))
            .map(|d| (*p, d))
    }))
}

fn filter_proposals(proposals: &HashMap<IVec2, IVec2>) -> HashMap<IVec2, IVec2> {
    HashMap::from_iter(
        proposals
            .iter()
            .filter(|&(_, d)| proposals.iter().filter(|&(_, d2)| *d == *d2).count() == 1)
            .map(|(p, d)| (*p, *d)),
    )
}

fn do_moves(all: &HashSet<IVec2>, moves: &HashMap<IVec2, IVec2>) -> HashSet<IVec2> {
    HashSet::from_iter(
        all.iter()
            .filter(|&p| !moves.contains_key(p))
            .cloned()
            .chain(moves.iter().map(|(_, p)| *p)),
    )
}

fn get_bounds(all: &HashSet<IVec2>) -> (IVec2, IVec2) {
    all.iter().fold(
        (IVec2::ONE * i32::MAX, IVec2::ONE * i32::MIN),
        |(min, max), p| (min.min(*p), max.max(*p)),
    )
}

fn num_empty_in_bounds(all: &HashSet<IVec2>, (min, max): (IVec2, IVec2)) -> usize {
    let size = (max - min) + 1;
    (size.x * size.y) as usize - all.len()
}

fn print(set: &HashSet<IVec2>, (min, max): (IVec2, IVec2)) {
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if set.contains(&ivec2(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part01(input: &str) -> usize {
    let initial = parse(input);
    print(&initial, get_bounds(&initial));

    let mut current = initial;
    let mut filters = vec![[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];
    for _ in 0..10 {
        let proposing = get_proposing(&current);
        let proposals = get_proposals(&current, &proposing, &filters);
        let moves = filter_proposals(&proposals);
        current = do_moves(&current, &moves);
        filters = Vec::from_iter((0..4).map(|i| filters[(i + 1) % 4]));

        print(&current, get_bounds(&current));
    }
    num_empty_in_bounds(&current, get_bounds(&current))
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let set = parse(SMALL_EXAMPLE);
        assert_eq!(set.len(), 5);
        assert!(set.contains(&ivec2(2, 1)));
        assert!(set.contains(&ivec2(3, 1)));
        assert!(set.contains(&ivec2(2, 2)));
        assert!(set.contains(&ivec2(2, 4)));
        assert!(set.contains(&ivec2(3, 4)));
    }

    #[test]
    fn test_has_adjacent() {
        let mut set = HashSet::new();
        set.insert(ivec2(1, 1));
        assert!(has_adjacent(ivec2(0, 0), &set));
        assert!(has_adjacent(ivec2(1, 0), &set));
        assert!(has_adjacent(ivec2(2, 0), &set));
        assert!(has_adjacent(ivec2(0, 1), &set));
        assert!(has_adjacent(ivec2(2, 1), &set));
        assert!(has_adjacent(ivec2(0, 2), &set));
        assert!(has_adjacent(ivec2(1, 2), &set));
        assert!(has_adjacent(ivec2(2, 2), &set));
        assert!(!has_adjacent(ivec2(3, 1), &set));
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 110);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
    }

    const EXAMPLE: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

    const SMALL_EXAMPLE: &str = r#".....
..##.
..#..
.....
..##.
....."#;
}
