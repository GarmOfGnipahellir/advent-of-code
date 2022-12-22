use std::{collections::HashMap, fmt::Display};

use glam::{ivec2, IVec2};

const INPUT: &str = include_str!("../inputs/22");

fn main() {
    println!("01: {}", part01(INPUT, 50));
    let mappings = &[];
    println!("02: {}", part02(INPUT, 50, mappings));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RotDir {
    Right,
    Left,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Move(i32),
    Rotate(RotDir),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn from_index(i: i32) -> Self {
        match i {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }

    fn to_index(&self) -> i32 {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }

    fn to_dir(&self) -> IVec2 {
        match self {
            Facing::Right => IVec2::X,
            Facing::Down => IVec2::Y,
            Facing::Left => IVec2::NEG_X,
            Facing::Up => IVec2::NEG_Y,
        }
    }

    fn rotate(&self, dir: RotDir) -> Self {
        let dir = match dir {
            RotDir::Right => 1,
            RotDir::Left => -1,
        };
        Self::from_index((4 + self.to_index() + dir) % 4)
    }
}

#[derive(Debug)]
enum Tile {
    Open,
    Solid,
}

#[derive(Debug)]
struct Path(Vec<Instruction>);

impl Path {
    fn parse(s: &str) -> Self {
        let mut res = Vec::new();
        let mut num_buf = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                num_buf.push(c);
            } else {
                res.push(Instruction::Move(num_buf.parse().unwrap()));
                num_buf.clear();
                let dir = match c {
                    'R' => RotDir::Right,
                    'L' => RotDir::Left,
                    _ => unreachable!(),
                };
                res.push(Instruction::Rotate(dir))
            }
        }
        res.push(Instruction::Move(num_buf.parse().unwrap()));
        Self(res)
    }
}

#[derive(Debug)]
struct Pawn {
    pos: IVec2,
    facing: Facing,
}

impl Pawn {
    fn rotate(&mut self, dir: RotDir) {
        self.facing = self.facing.rotate(dir);
    }
}

#[derive(Debug)]
struct Mapping {
    from: IVec2,
    to: IVec2,
    rot: Option<RotDir>,
}

impl From<(IVec2, IVec2)> for Mapping {
    fn from((from, to): (IVec2, IVec2)) -> Self {
        Self {
            from,
            to,
            rot: None,
        }
    }
}

impl From<(IVec2, IVec2, RotDir)> for Mapping {
    fn from((from, to, rot): (IVec2, IVec2, RotDir)) -> Self {
        Self {
            from,
            to,
            rot: Some(rot),
        }
    }
}

#[derive(Debug)]
struct Board {
    tiles: HashMap<IVec2, Tile>,
    size: IVec2,
    chunk_size: i32,
}

impl Board {
    fn parse(s: &str, chunk_size: i32) -> Self {
        let mut tiles = HashMap::new();
        let mut size = IVec2::ZERO;
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let pos = ivec2(x as i32, y as i32);
                let tile = match c {
                    '.' => Tile::Open,
                    '#' => Tile::Solid,
                    ' ' => continue,
                    _ => unreachable!(),
                };
                tiles.insert(pos, tile);
                size = size.max(pos);
            }
        }
        Self {
            tiles,
            size,
            chunk_size,
        }
    }

    fn is_open(&self, p: IVec2) -> bool {
        if let Some(Tile::Open) = self.tiles.get(&p) {
            true
        } else {
            false
        }
    }

    fn is_solid(&self, p: IVec2) -> bool {
        if let Some(Tile::Solid) = self.tiles.get(&p) {
            true
        } else {
            false
        }
    }

    fn is_empty(&self, p: IVec2) -> bool {
        self.tiles.get(&p).is_none()
    }

    fn wrap_pos_cartesian(&self, pos: IVec2, facing: Facing) -> IVec2 {
        if !self.is_empty(pos) {
            return pos;
        }
        let mut pos = pos;
        let dir = -facing.to_dir();
        while !self.is_empty(pos + dir) {
            pos += dir;
        }
        pos
    }

    fn chunk(&self, pos: IVec2) -> IVec2 {
        pos / self.chunk_size
    }

    fn to_local(&self, pos: IVec2) -> IVec2 {
        pos - (pos / self.chunk_size) * self.chunk_size
    }

    fn wrap_pos_cube(&self, pos: IVec2, facing: Facing, mappings: &[Mapping]) -> (IVec2, Facing) {
        if !self.is_empty(pos) {
            return (pos, facing);
        }
        let mut pos = pos;
        let mut facing = facing;
        let from = self.chunk(pos);
        let mapping = mappings.iter().find(|&m| m.from == from).unwrap();
        let mut local = self.to_local(pos);
        println!("from: {local:?}");
        if let Some(dir) = mapping.rot {
            let mut dir = dir;
            if facing != facing.rotate(dir) {
                dir = match dir {
                    RotDir::Right => RotDir::Left,
                    RotDir::Left => RotDir::Right,
                };
            };

            local = match dir {
                RotDir::Right => ivec2((self.chunk_size - 1) - local.y, local.x),
                RotDir::Left => ivec2(local.y, (self.chunk_size - 1) - local.x),
            };
            println!("to: {local:?}");
            facing = facing.rotate(dir);
        }
        pos = mapping.to * self.chunk_size + local;
        // TODO: wrapping logic
        (pos, facing)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.size.y {
            for x in 0..=self.size.x {
                let pos = ivec2(x, y);
                if let Some(tile) = self.tiles.get(&pos) {
                    match tile {
                        Tile::Open => write!(f, ".")?,
                        Tile::Solid => write!(f, "#")?,
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part01(input: &str, chunk_size: i32) -> i32 {
    let mut lines = input.lines();
    let path = Path::parse(lines.next_back().unwrap());
    let board = Board::parse(&lines.collect::<Vec<_>>().join("\n"), chunk_size);
    let mut top_left = IVec2::ZERO;
    for x in 0..=board.size.x {
        if board.is_open(ivec2(x, 0)) {
            top_left.x = x;
        }
    }
    let mut pawn = Pawn {
        pos: top_left,
        facing: Facing::Right,
    };
    for instr in path.0 {
        match instr {
            Instruction::Move(dist) => {
                for _ in 0..dist {
                    let dir = pawn.facing.to_dir();
                    let pos = board.wrap_pos_cartesian(pawn.pos + dir, pawn.facing);
                    if !board.is_solid(pos) {
                        pawn.pos = pos;
                    }
                }
            }
            Instruction::Rotate(dir) => pawn.rotate(dir),
        }
    }
    (pawn.pos.y + 1) * 1000 + (pawn.pos.x + 1) * 4 + pawn.facing.to_index()
}

fn part02(input: &str, chunk_size: i32, mappings: &[Mapping]) -> i32 {
    let mut lines = input.lines();
    let path = Path::parse(lines.next_back().unwrap());
    let board = Board::parse(&lines.collect::<Vec<_>>().join("\n"), chunk_size);
    let mut top_left = IVec2::ZERO;
    for x in 0..=board.size.x {
        if board.is_open(ivec2(x, 0)) {
            top_left.x = x;
        }
    }
    let mut pawn = Pawn {
        pos: top_left,
        facing: Facing::Right,
    };
    for instr in path.0 {
        match instr {
            Instruction::Move(dist) => {
                for _ in 0..dist {
                    let dir = pawn.facing.to_dir();
                    let (pos, facing) = board.wrap_pos_cube(pawn.pos + dir, pawn.facing, mappings);
                    if !board.is_solid(pos) {
                        pawn.pos = pos;
                        pawn.facing = facing;
                    }
                }
            }
            Instruction::Rotate(dir) => pawn.rotate(dir),
        }
    }
    (pawn.pos.y + 1) * 1000 + (pawn.pos.x + 1) * 4 + pawn.facing.to_index()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facing_rotate() {
        assert_eq!(Facing::Right.rotate(RotDir::Right), Facing::Down);
        assert_eq!(Facing::Right.rotate(RotDir::Left), Facing::Up);
        assert_eq!(Facing::Down.rotate(RotDir::Right), Facing::Left);
        assert_eq!(Facing::Down.rotate(RotDir::Left), Facing::Right);
        assert_eq!(Facing::Left.rotate(RotDir::Right), Facing::Up);
        assert_eq!(Facing::Left.rotate(RotDir::Left), Facing::Down);
        assert_eq!(Facing::Up.rotate(RotDir::Right), Facing::Right);
        assert_eq!(Facing::Up.rotate(RotDir::Left), Facing::Left);
    }

    #[test]
    fn test_path_parse() {
        let path = Path::parse("10R5L5R10L4R5L55");
        assert_eq!(
            path.0,
            vec![
                Instruction::Move(10),
                Instruction::Rotate(RotDir::Right),
                Instruction::Move(5),
                Instruction::Rotate(RotDir::Left),
                Instruction::Move(5),
                Instruction::Rotate(RotDir::Right),
                Instruction::Move(10),
                Instruction::Rotate(RotDir::Left),
                Instruction::Move(4),
                Instruction::Rotate(RotDir::Right),
                Instruction::Move(5),
                Instruction::Rotate(RotDir::Left),
                Instruction::Move(55),
            ]
        )
    }

    #[test]
    fn test_wrap_pos_cube() {
        let mappings = &[(ivec2(3, 1), ivec2(3, 2), RotDir::Right).into()];
        let board = Board::parse(EX_BOARD, 4);
        assert_eq!(
            board.wrap_pos_cube(ivec2(12, 5), Facing::Right, mappings),
            (ivec2(14, 8), Facing::Down)
        );
        assert_eq!(
            board.wrap_pos_cube(ivec2(14, 8), Facing::Up, mappings),
            (ivec2(12, 5), Facing::Left)
        );
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE, 4), 6032);
    }

    #[test]
    fn example02() {
        let mappings = &[];
        assert_eq!(part02(EXAMPLE, 4, mappings), -1);
    }

    const EXAMPLE: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    const EX_BOARD: &str = r#"        ...#
    .#..
    #...
    ....
...#.......#
........#...
..#....#....
..........#.
    ...#....
    .....#..
    .#......
    ......#."#;
}
