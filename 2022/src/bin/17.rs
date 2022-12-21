use std::{cell::RefCell, collections::HashSet};

use glam::{ivec2, IVec2};

const INPUT: &str = include_str!("../inputs/17");

fn main() {
    println!("{}, {}", INPUT.trim().len(), INPUT.trim().len() as f32 / 5.0);
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

const ROCK_SHAPES: [&str; 5] = [
    "####",
    r#".#.
###
.#."#,
    r#"..#
..#
###"#,
    r#"#
#
#
#"#,
    r#"##
##"#,
];

#[derive(Debug, PartialEq, Clone)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }

    fn to_ivec2(&self) -> IVec2 {
        match self {
            Jet::Left => IVec2::NEG_X,
            Jet::Right => IVec2::X,
        }
    }
}

#[derive(Debug, PartialEq)]
struct JetPattern(Vec<Jet>);

impl JetPattern {
    fn parse(s: &str) -> Self {
        Self(s.trim().chars().map(|c| Jet::from_char(c)).collect())
    }

    fn iter(&self) -> JetIter {
        JetIter {
            jet_pattern: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Shape {
    points: HashSet<IVec2>,
    size: IVec2,
}

impl Shape {
    fn parse(s: &str) -> Self {
        let points = HashSet::from_iter(s.lines().rev().enumerate().flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(ivec2(x as i32, y as i32))
                } else {
                    None
                }
            })
        }));
        let size = points.iter().fold(IVec2::NEG_ONE * i32::MAX, |acc, p| {
            ivec2(i32::max(acc.x, p.x), i32::max(acc.y, p.y))
        });
        Self { points, size }
    }
}

#[derive(Debug, PartialEq)]
struct Rock {
    shape: Shape,
    pos: IVec2,
}

impl Rock {
    fn new(shape: Shape, pos: IVec2) -> Self {
        Self { shape, pos }
    }

    fn shape_points_world(&self) -> HashSet<IVec2> {
        HashSet::from_iter(self.shape.points.iter().map(|p| *p + self.pos))
    }
}

#[derive(Debug, Default)]
struct Chamber {
    blocked: RefCell<HashSet<IVec2>>,
}

impl Chamber {
    fn highest_point(&self) -> i32 {
        if self.blocked.borrow().is_empty() {
            0
        } else {
            self.blocked.borrow().iter().map(|p| p.y).max().unwrap() + 1
        }
    }

    fn rock_iter(&self, shapes: &[&str]) -> RockIter {
        RockIter {
            shapes: shapes.iter().map(|&s| Shape::parse(s)).collect(),
            index: 0,
            chamber: self,
        }
    }

    fn add_blocking_rock(&self, rock: &Rock) {
        for point in rock.shape.points.iter().map(|p| *p + rock.pos) {
            self.blocked.borrow_mut().insert(point);
        }
    }

    fn is_blocked(&self, pos: IVec2) -> bool {
        if pos.x < 0 || pos.x >= 7 || pos.y < 0 {
            return true;
        }
        self.blocked.borrow().contains(&pos)
    }

    fn can_move(&self, rock: &Rock, dir: IVec2) -> bool {
        rock.shape_points_world()
            .iter()
            .all(|p| !self.is_blocked(*p + dir))
    }

    fn drop_rock(&self, mut rock: Rock, jet_iter: &mut JetIter) {
        loop {
            let jet_dir = jet_iter.next().unwrap().to_ivec2();
            if self.can_move(&rock, jet_dir) {
                rock.pos += jet_dir;
            }
            if self.can_move(&rock, IVec2::NEG_Y) {
                rock.pos += IVec2::NEG_Y;
            } else {
                break;
            }
        }
        self.add_blocking_rock(&rock);
    }
}

struct RockIter<'a> {
    shapes: Vec<Shape>,
    index: usize,
    chamber: &'a Chamber,
}

impl<'a> Iterator for RockIter<'a> {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let shape = &self.shapes[self.index];
        let pos = ivec2(2, self.chamber.highest_point() + 3);
        self.index = (self.index + 1) % self.shapes.len();
        Some(Rock::new(shape.clone(), pos))
    }
}

struct JetIter<'a> {
    jet_pattern: &'a JetPattern,
    index: usize,
}

impl<'a> Iterator for JetIter<'a> {
    type Item = Jet;

    fn next(&mut self) -> Option<Self::Item> {
        let res = &self.jet_pattern.0[self.index];
        self.index = (self.index + 1) % self.jet_pattern.0.len();
        Some(res.clone())
    }
}

fn part01(input: &str) -> i32 {
    let jet_pattern = JetPattern::parse(input);
    let chamber = Chamber::default();
    let mut rock_iter = chamber.rock_iter(&ROCK_SHAPES);
    let mut jet_iter = jet_pattern.iter();
    for _ in 0..2022 {
        chamber.drop_rock(rock_iter.next().unwrap(), &mut jet_iter);
    }

    for y in (0..chamber.highest_point()).rev() {
        for x in 0..7 {
            if chamber.is_blocked(ivec2(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    chamber.highest_point()
}

fn part02(input: &str) -> i64 {
    let jet_pattern = JetPattern::parse(input);
    let chamber = Chamber::default();
    let mut rock_iter = chamber.rock_iter(&ROCK_SHAPES);
    let mut jet_iter = jet_pattern.iter();
    for _ in 0..1_000_000_000_000_i64 {
        chamber.drop_rock(rock_iter.next().unwrap(), &mut jet_iter);
    }
    chamber.highest_point() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Chamber {
        fn new(blocked: &[IVec2]) -> Self {
            Self {
                blocked: RefCell::new(HashSet::from_iter(blocked.iter().cloned())),
            }
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            JetPattern::parse("<>"),
            JetPattern(vec![Jet::Left, Jet::Right])
        );
        assert_eq!(
            JetPattern::parse("<<>"),
            JetPattern(vec![Jet::Left, Jet::Left, Jet::Right])
        );

        let shape = Shape::parse(ROCK_SHAPES[0]);
        let points = [ivec2(0, 0), ivec2(1, 0), ivec2(2, 0), ivec2(3, 0)];
        assert_eq!(points.len(), shape.points.len());
        assert!(points.iter().all(|p| shape.points.contains(p)));

        let shape = Shape::parse(ROCK_SHAPES[2]);
        let points = [
            ivec2(0, 0),
            ivec2(1, 0),
            ivec2(2, 0),
            ivec2(2, 1),
            ivec2(2, 2),
        ];
        assert_eq!(points.len(), shape.points.len());
        assert!(points.iter().all(|p| shape.points.contains(p)));
    }

    #[test]
    fn test_highest_point() {
        assert_eq!(Chamber::default().highest_point(), 0);
        assert_eq!(Chamber::new(&[ivec2(0, 1)]).highest_point(), 1);
        assert_eq!(Chamber::new(&[ivec2(7, 10)]).highest_point(), 10);
    }

    #[test]
    fn test_rock_iter() {
        let chamber = Chamber::default();
        let rock_shapes = &["#", "##", ".#"];
        let mut iter = chamber.rock_iter(rock_shapes);
        assert_eq!(
            iter.next().unwrap(),
            Rock::new(Shape::parse(rock_shapes[0]), ivec2(2, 3))
        );
        assert_eq!(
            iter.next().unwrap(),
            Rock::new(Shape::parse(rock_shapes[1]), ivec2(2, 3))
        );
        assert_eq!(
            iter.next().unwrap(),
            Rock::new(Shape::parse(rock_shapes[2]), ivec2(2, 3))
        );
        assert_eq!(
            iter.next().unwrap(),
            Rock::new(Shape::parse(rock_shapes[0]), ivec2(2, 3))
        );
    }

    #[test]
    fn test_jet_iter() {
        let jet_pattern = JetPattern::parse("<<>");
        let mut iter = jet_pattern.iter();
        assert_eq!(iter.next().unwrap(), Jet::Left);
        assert_eq!(iter.next().unwrap(), Jet::Left);
        assert_eq!(iter.next().unwrap(), Jet::Right);
        assert_eq!(iter.next().unwrap(), Jet::Left);
    }

    #[test]
    fn test_add_blocking_rock() {
        let mut chamber = Chamber::default();
        chamber.add_blocking_rock(&Rock::new(Shape::parse(ROCK_SHAPES[2]), ivec2(2, 3)));
        assert!(chamber.is_blocked(ivec2(2, 3)));
        assert!(chamber.is_blocked(ivec2(3, 3)));
        assert!(chamber.is_blocked(ivec2(4, 3)));
        assert!(!chamber.is_blocked(ivec2(2, 4)));
        assert!(!chamber.is_blocked(ivec2(3, 4)));
        assert!(chamber.is_blocked(ivec2(4, 4)));
        assert!(!chamber.is_blocked(ivec2(2, 5)));
        assert!(!chamber.is_blocked(ivec2(3, 5)));
        assert!(chamber.is_blocked(ivec2(4, 5)));
    }

    #[test]
    fn test_drop_rock() {
        let chamber = Chamber::default();
        let mut rock_iter = chamber.rock_iter(&ROCK_SHAPES);
        let jet_pattern = JetPattern::parse(EXAMPLE);
        let mut jet_iter = jet_pattern.iter();
        chamber.drop_rock(rock_iter.next().unwrap(), &mut jet_iter);
        chamber.drop_rock(rock_iter.next().unwrap(), &mut jet_iter);
        chamber.drop_rock(rock_iter.next().unwrap(), &mut jet_iter);

        assert!(!chamber.is_blocked(ivec2(0, 0)));
        assert!(!chamber.is_blocked(ivec2(1, 0)));
        assert!(chamber.is_blocked(ivec2(2, 0)));
        assert!(chamber.is_blocked(ivec2(3, 0)));
        assert!(chamber.is_blocked(ivec2(4, 0)));
        assert!(chamber.is_blocked(ivec2(5, 0)));
        assert!(!chamber.is_blocked(ivec2(6, 0)));

        assert!(!chamber.is_blocked(ivec2(0, 1)));
        assert!(!chamber.is_blocked(ivec2(1, 1)));
        assert!(!chamber.is_blocked(ivec2(2, 1)));
        assert!(chamber.is_blocked(ivec2(3, 1)));
        assert!(!chamber.is_blocked(ivec2(4, 1)));
        assert!(!chamber.is_blocked(ivec2(5, 1)));
        assert!(!chamber.is_blocked(ivec2(6, 1)));

        assert!(!chamber.is_blocked(ivec2(0, 2)));
        assert!(!chamber.is_blocked(ivec2(1, 2)));
        assert!(chamber.is_blocked(ivec2(2, 2)));
        assert!(chamber.is_blocked(ivec2(3, 2)));
        assert!(chamber.is_blocked(ivec2(4, 2)));
        assert!(!chamber.is_blocked(ivec2(5, 2)));
        assert!(!chamber.is_blocked(ivec2(6, 2)));

        assert!(chamber.is_blocked(ivec2(0, 3)));
        assert!(chamber.is_blocked(ivec2(1, 3)));
        assert!(chamber.is_blocked(ivec2(2, 3)));
        assert!(chamber.is_blocked(ivec2(3, 3)));
        assert!(!chamber.is_blocked(ivec2(4, 3)));
        assert!(!chamber.is_blocked(ivec2(5, 3)));
        assert!(!chamber.is_blocked(ivec2(6, 3)));

        assert!(!chamber.is_blocked(ivec2(0, 4)));
        assert!(!chamber.is_blocked(ivec2(1, 4)));
        assert!(chamber.is_blocked(ivec2(2, 4)));
        assert!(!chamber.is_blocked(ivec2(3, 4)));
        assert!(!chamber.is_blocked(ivec2(4, 4)));
        assert!(!chamber.is_blocked(ivec2(5, 4)));
        assert!(!chamber.is_blocked(ivec2(6, 4)));

        assert!(!chamber.is_blocked(ivec2(0, 5)));
        assert!(!chamber.is_blocked(ivec2(1, 5)));
        assert!(chamber.is_blocked(ivec2(2, 5)));
        assert!(!chamber.is_blocked(ivec2(3, 5)));
        assert!(!chamber.is_blocked(ivec2(4, 5)));
        assert!(!chamber.is_blocked(ivec2(5, 5)));
        assert!(!chamber.is_blocked(ivec2(6, 5)));
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 3068);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 1514285714288);
    }

    const EXAMPLE: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
}
