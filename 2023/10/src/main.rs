const INP: &str = include_str!("../in.txt");

struct Vert {
    edge: usize,
}

struct Edge {
    from: usize,
    to: usize,
}

struct Graph {
    verts: Vec<Vert>,
    edges: Vec<Edge>,
}

impl Graph {
    fn parse(inp: &str) -> Self {
        todo!()
    }
}

fn part1(inp: &str) -> usize {
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
