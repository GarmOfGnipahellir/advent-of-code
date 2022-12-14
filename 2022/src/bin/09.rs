use glam::IVec2;
use std::collections::VecDeque;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/09")));
    println!("02: {}", part02(include_str!("../inputs/09")));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Into<IVec2> for Direction {
    fn into(self) -> IVec2 {
        match self {
            Direction::Right => IVec2::X,
            Direction::Left => IVec2::NEG_X,
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
        }
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: IVec2,
    tail: IVec2,
}

impl Rope {
    fn new(head: IVec2, tail: IVec2) -> Self {
        Self { head, tail }
    }

    fn is_touching(&self) -> bool {
        let delta = self.head - self.tail;
        delta.x.abs() < 2 && delta.y.abs() < 2
    }

    fn step_tail(&mut self) {
        if self.is_touching() {
            return;
        }

        self.tail += (self.head - self.tail).signum();
    }

    fn step_head(&mut self, direction: Direction) {
        self.head += Into::<IVec2>::into(direction);
    }

    fn step(&mut self, direction: Direction) {
        self.step_head(direction);
        self.step_tail();
    }
}

fn part01(input: &str) -> usize {
    let mut direction_queue = VecDeque::<Direction>::new();
    for line in input.lines() {
        let (d, n) = line.split_once(' ').unwrap();
        let n = n.parse::<usize>().unwrap();
        let d = match d {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };

        direction_queue.extend(vec![d; n].iter());
    }
    let mut rope = Rope::default();
    let mut unique_tail_positions = Vec::new();
    for direction in direction_queue {
        rope.step(direction);

        if !unique_tail_positions.contains(&rope.tail) {
            unique_tail_positions.push(rope.tail);
        }
    }
    unique_tail_positions.len()
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_touching() {
        assert!(Rope::new(IVec2::ZERO, IVec2::ZERO).is_touching());

        assert!(Rope::new(IVec2::ZERO, IVec2::new(0, 1)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(1, 0)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(0, -1)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(-1, 0)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(1, 1)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(-1, -1)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(1, -1)).is_touching());
        assert!(Rope::new(IVec2::ZERO, IVec2::new(-1, 1)).is_touching());

        assert!(!Rope::new(IVec2::ZERO, IVec2::new(0, 2)).is_touching());
        assert!(!Rope::new(IVec2::ZERO, IVec2::new(2, 0)).is_touching());
        assert!(!Rope::new(IVec2::ZERO, IVec2::new(1, 2)).is_touching());
    }

    #[test]
    fn test_step_tail() {
        let mut rope = Rope::new(IVec2::ZERO, IVec2::new(2, 0));
        rope.step_tail();
        assert_eq!(rope.tail, IVec2::new(1, 0));

        let mut rope = Rope::new(IVec2::ZERO, IVec2::new(0, 2));
        rope.step_tail();
        assert_eq!(rope.tail, IVec2::new(0, 1));

        let mut rope = Rope::new(IVec2::ZERO, IVec2::new(-1, -2));
        rope.step_tail();
        assert_eq!(rope.tail, IVec2::new(0, -1));

        let mut rope = Rope::new(IVec2::ZERO, IVec2::new(-2, -1));
        rope.step_tail();
        assert_eq!(rope.tail, IVec2::new(-1, 0));
    }

    #[test]
    fn example01() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        assert_eq!(part01(input), 13);
    }

    #[test]
    fn example02() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        assert_eq!(part02(input), -1);
    }
}
