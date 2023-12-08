const INP: &str = include_str!("../in.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    vertices: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn parse(inp: &str) -> Self {
        let (vertices, edges) = inp
            .trim()
            .lines()
            .map(|line| {
                let (head, tail) = line.trim().split_once('=').unwrap();
                let vert = head.trim();
                let (head, tail) = tail.trim().split_once(',').unwrap();
                let left = head.trim().strip_prefix('(').unwrap();
                let right = tail.trim().strip_suffix(')').unwrap();
                (vert, (left, right))
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let edges = edges
            .into_iter()
            .map(|(l, r)| {
                let left = vertices
                    .iter()
                    .enumerate()
                    .find_map(|(i, &v)| (l == v).then_some(i))
                    .unwrap();
                let right = vertices
                    .iter()
                    .enumerate()
                    .find_map(|(i, &v)| (r == v).then_some(i))
                    .unwrap();
                (left, right)
            })
            .collect::<Vec<_>>();

        let vertices = vertices.into_iter().map(|s| s.to_string()).collect();

        Self { vertices, edges }
    }

    fn find(&self, name: &str) -> usize {
        self.vertices
            .iter()
            .enumerate()
            .find_map(|(i, v)| (name == v).then_some(i))
            .unwrap()
    }

    fn step(&self, from: usize, dir: Direction) -> usize {
        match dir {
            Direction::Left => self.edges[from].0,
            Direction::Right => self.edges[from].1,
        }
    }
}

fn parse_input(inp: &str) -> (Vec<Direction>, Graph) {
    let (head, tail) = inp
        .split_once("\n\n")
        .or_else(|| inp.split_once("\r\n\r\n"))
        .unwrap();
    let directions = head
        .trim()
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    let graph = Graph::parse(tail);
    (directions, graph)
}

fn part1(inp: &str) -> usize {
    let (directions, graph) = parse_input(inp);
    let mut step = 0;
    let mut idx = graph.find("AAA");
    loop {
        idx = graph.step(idx, directions[step % directions.len()]);
        step += 1;
        if graph.vertices[idx] == "ZZZ".to_string() {
            break;
        }
    }
    step
}

fn part2(inp: &str) -> usize {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(INP));
    // println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const EX2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn test_graph_parse() {
        assert_eq!(
            Graph::parse(
                r#"AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"#
            ),
            Graph {
                vertices: vec!["AAA".to_string(), "BBB".to_string(), "ZZZ".to_string()],
                edges: vec![(1, 1), (0, 2), (2, 2)]
            }
        );
    }

    #[test]
    fn test_graph_find() {
        let graph = Graph::parse(
            r#"AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)"#,
        );
        assert_eq!(graph.find("AAA"), 0);
        assert_eq!(graph.find("BBB"), 1);
        assert_eq!(graph.find("ZZZ"), 2);
    }

    #[test]
    fn test_graph_step() {
        let graph = Graph::parse(
            r#"AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)"#,
        );
        assert_eq!(graph.step(0, Direction::Left), 1);
        assert_eq!(graph.step(0, Direction::Right), 1);
        assert_eq!(graph.step(1, Direction::Left), 0);
        assert_eq!(graph.step(1, Direction::Right), 2);
        assert_eq!(graph.step(2, Direction::Left), 2);
        assert_eq!(graph.step(2, Direction::Right), 2);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EX2),
            (
                vec![Direction::Left, Direction::Left, Direction::Right],
                Graph::parse(
                    r#"AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"#
                )
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 2);
        assert_eq!(part1(EX2), 6);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(EX1), 0);
    }
}
