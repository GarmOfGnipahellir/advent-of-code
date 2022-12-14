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
    knots: Vec<IVec2>,
}

impl Rope {
    fn new(knots: Vec<IVec2>) -> Self {
        Self { knots }
    }

    fn new_short(head: IVec2, tail: IVec2) -> Self {
        Self {
            knots: vec![head, tail],
        }
    }

    fn is_touching(&self, i: usize) -> bool {
        let delta = self.knots[i - 1] - self.knots[i];
        delta.x.abs() < 2 && delta.y.abs() < 2
    }

    fn step_knot(&mut self, i: usize) {
        if self.is_touching(i) {
            return;
        }

        let delta = self.knots[i - 1] - self.knots[i];
        self.knots[i] += delta.signum();
    }

    fn step_head(&mut self, direction: Direction) {
        self.knots[0] += Into::<IVec2>::into(direction);
    }

    fn step(&mut self, direction: Direction) {
        self.step_head(direction);
        for i in 1..self.knots.len() {
            self.step_knot(i);
        }
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
    let mut rope = Rope::new_short(IVec2::ZERO, IVec2::ZERO);
    let mut unique_tail_positions = Vec::new();
    for direction in direction_queue {
        rope.step(direction);

        if !unique_tail_positions.contains(&rope.knots[1]) {
            unique_tail_positions.push(rope.knots[1]);
        }
    }
    unique_tail_positions.len()
}

fn part02(input: &str) -> usize {
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
    let mut rope = Rope::new(vec![IVec2::ZERO; 10]);
    let mut unique_tail_positions = Vec::new();
    for direction in direction_queue {
        rope.step(direction);

        if !unique_tail_positions.contains(&rope.knots[9]) {
            unique_tail_positions.push(rope.knots[9]);
        }
    }
    unique_tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_touching() {
        assert!(Rope::new_short(IVec2::ZERO, IVec2::ZERO).is_touching(1));

        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(0, 1)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(1, 0)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(0, -1)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(-1, 0)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(1, 1)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(-1, -1)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(1, -1)).is_touching(1));
        assert!(Rope::new_short(IVec2::ZERO, IVec2::new(-1, 1)).is_touching(1));

        assert!(!Rope::new_short(IVec2::ZERO, IVec2::new(0, 2)).is_touching(1));
        assert!(!Rope::new_short(IVec2::ZERO, IVec2::new(2, 0)).is_touching(1));
        assert!(!Rope::new_short(IVec2::ZERO, IVec2::new(1, 2)).is_touching(1));
    }

    #[test]
    fn test_step_tail() {
        let mut rope = Rope::new_short(IVec2::ZERO, IVec2::new(2, 0));
        rope.step_knot(1);
        assert_eq!(rope.knots[1], IVec2::new(1, 0));

        let mut rope = Rope::new_short(IVec2::ZERO, IVec2::new(0, 2));
        rope.step_knot(1);
        assert_eq!(rope.knots[1], IVec2::new(0, 1));

        let mut rope = Rope::new_short(IVec2::ZERO, IVec2::new(-1, -2));
        rope.step_knot(1);
        assert_eq!(rope.knots[1], IVec2::new(0, -1));

        let mut rope = Rope::new_short(IVec2::ZERO, IVec2::new(-2, -1));
        rope.step_knot(1);
        assert_eq!(rope.knots[1], IVec2::new(-1, 0));
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
        assert_eq!(part02(input), 1);
    }

    #[test]
    fn example03() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        assert_eq!(part02(input), 36);
    }
}
