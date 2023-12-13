use std::fmt::{self, Write};

const INP: &str = include_str!("../in.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Ground,
    PipeNS,
    PipeEW,
    PipeNE,
    PipeNW,
    PipeSW,
    PipeSE,
    Start,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Cell::Ground => '◦',
            Cell::PipeNS => '│',
            Cell::PipeEW => '─',
            Cell::PipeNE => '└',
            Cell::PipeNW => '┘',
            Cell::PipeSW => '┐',
            Cell::PipeSE => '┌',
            Cell::Start => '■',
        })
    }
}

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Ground,
            '|' => Self::PipeNS,
            '-' => Self::PipeEW,
            'L' => Self::PipeNE,
            'J' => Self::PipeNW,
            '7' => Self::PipeSW,
            'F' => Self::PipeSE,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[x + y * self.width];
                write!(f, "{cell}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn parse(inp: &str) -> Self {
        let width = inp.lines().next().unwrap().len();
        let height = inp.lines().count();
        let cells = inp
            .replace("\n", "")
            .replace("\r", "")
            .replace(" ", "")
            .chars()
            .map(|ch| Cell::from_char(ch))
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    fn get_cell(&self, pos: (usize, usize)) -> Cell {
        self.cells[pos.0 + pos.1 * self.width]
    }

    fn find_start(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[x + y * self.width];
                if cell == Cell::Start {
                    return (x, y);
                }
            }
        }
        unreachable!()
    }

    fn step_from(&self, from: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
        todo!()
    }

    fn loop_len(&self) -> usize {
        let start = self.find_start();
        let mut curr = start;
        let mut prev = start;
        let mut n = 0;
        loop {
            let next = self.step_from(curr, prev);
            if next == start {
                break;
            }

            prev = curr;
            curr = next;
            n += 1;
        }
        n
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    println!("{map}");
    todo!()
}

fn part2(inp: &str) -> usize {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(INP));
    println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

    const EX2: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    #[test]
    fn test_map_parse() {
        assert_eq!(
            Map::parse(EX1),
            Map {
                width: 5,
                height: 5,
                cells: vec![
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::PipeSE,
                    Cell::PipeEW,
                    Cell::PipeSW,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::PipeNS,
                    Cell::Ground,
                    Cell::PipeNS,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::PipeNE,
                    Cell::PipeEW,
                    Cell::PipeNW,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                    Cell::Ground,
                ],
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 4);
        assert_eq!(part1(EX2), 8);
    }

    #[test]
    #[ignore = "not implemented"]
    fn test_part2() {
        assert_eq!(part2(EX1), 0);
    }
}
