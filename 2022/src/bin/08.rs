fn main() {
    println!("01: {}", part01(include_str!("../inputs/08")));
    println!("02: {}", part02(include_str!("../inputs/08")));
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Map {
    width: usize,
    height: usize,
    trees: Vec<u32>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let (width, height) = { (s.lines().count(), s.lines().next().unwrap().len()) };
        let trees = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();
        Self {
            width,
            height,
            trees,
        }
    }

    fn calc_visibility(&self) -> Vec<bool> {
        for (i, tree) in self.trees.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
        }
        Vec::new()
    }

    fn index_to_coord(&self, i: usize) -> Coord {
        
    }
}

fn part01(input: &str) -> i32 {
    let map = Map::parse(input);
    println!("{:?}", map.trees);
    map.calc_visibility();
    unimplemented!()
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(part01(input), -1);
    }

    #[test]
    fn example02() {
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(part02(input), -1);
    }
}
