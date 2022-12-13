use std::fmt::Display;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/08")));
    println!("02: {}", part02(include_str!("../inputs/08")));
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn height(&self, coord: Coord) -> u32 {
        self.trees[self.coord_to_index(coord)]
    }

    fn calc_visibility(&self) -> Vec<bool> {
        let mut res = vec![false; self.trees.len()];
        for (i, &tree) in self.trees.iter().enumerate() {
            let coord = self.index_to_coord(i);

            if coord.x == 0
                || coord.x == self.width - 1
                || coord.y == 0
                || coord.y == self.height - 1
            {
                res[i] = true;
                continue;
            }

            let mut vis_left = true;
            for x in 0..coord.x {
                if self.height(Coord::new(x, coord.y)) >= tree {
                    vis_left = false;
                    break;
                }
            }
            let mut vis_right = true;
            for x in coord.x + 1..self.width {
                if self.height(Coord::new(x, coord.y)) >= tree {
                    vis_right = false;
                    break;
                }
            }
            let mut vis_top = true;
            for y in 0..coord.y {
                if self.height(Coord::new(coord.x, y)) >= tree {
                    vis_top = false;
                    break;
                }
            }
            let mut vis_bot = true;
            for y in coord.y + 1..self.height {
                if self.height(Coord::new(coord.x, y)) >= tree {
                    vis_bot = false;
                    break;
                }
            }

            res[i] = vis_left || vis_right || vis_top || vis_bot;
        }
        res
    }

    fn index_to_coord(&self, i: usize) -> Coord {
        Coord::new(i % self.width, i / self.width)
    }

    fn coord_to_index(&self, c: Coord) -> usize {
        c.x + c.y * self.width
    }
}

fn print_map<T>(v: &Vec<T>, width: usize)
where
    T: Display,
{
    println!(
        "{}",
        v.iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .chunks(width)
            .map(|chunk| chunk.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn part01(input: &str) -> i32 {
    let map = Map::parse(input);
    print_map(&map.trees, map.width);
    let visible = map
        .calc_visibility()
        .iter()
        .map(|&v| if v { 1 } else { 0 })
        .collect();
    println!("");
    print_map(&visible, map.width);
    visible.iter().sum()
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords() {
        let input = r#"30373
25512
65332
33549
35390"#;
        let map = Map::parse(input);
        assert_eq!(map.height(Coord::new(0, 0)), 3);
        assert_eq!(map.height(Coord::new(0, 1)), 2);
        assert_eq!(map.height(Coord::new(0, 2)), 6);
        assert_eq!(map.height(Coord::new(0, 3)), 3);
        assert_eq!(map.height(Coord::new(0, 4)), 3);
        assert_eq!(map.height(Coord::new(1, 0)), 0);
        assert_eq!(map.height(Coord::new(1, 1)), 5);
        assert_eq!(map.height(Coord::new(1, 2)), 5);
        assert_eq!(map.height(Coord::new(1, 3)), 3);
        assert_eq!(map.height(Coord::new(1, 4)), 5);

        assert_eq!(map.index_to_coord(0), Coord::new(0, 0));
        assert_eq!(map.index_to_coord(5), Coord::new(0, 1));
    }

    #[test]
    fn example01() {
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(part01(input), 21);
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
