fn main() {
    println!("01: {}", part01(include_str!("../inputs/20")));
    println!("02: {}", part02(include_str!("../inputs/20")));
}

#[derive(Debug, PartialEq)]
struct List {
    source: Vec<i32>,
    index_map: Vec<usize>,
}

impl List {
    fn new(source: Vec<i32>) -> Self {
        let index_map = (0..source.len()).collect();
        Self { source, index_map }
    }

    fn parse(s: &str) -> Self {
        let source = s
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<_>>();
        Self::new(source)
    }

    fn len(&self) -> usize {
        self.source.len()
    }

    fn value(&self, index: usize) -> i32 {
        self.source[self.index_map[index]]
    }

    fn value_after_zero(&self, index: usize) -> i32 {
        let zero_source_index = self
            .source
            .iter()
            .enumerate()
            .find_map(|(k, v)| if *v == 0 { Some(k) } else { None })
            .unwrap();
        let zero_index = self.index(zero_source_index);
        self.value(zero_index + index)
    }

    fn index(&self, source: usize) -> usize {
        self.index_map
            .iter()
            .enumerate()
            .find_map(|(k, v)| if *v == source { Some(k) } else { None })
            .unwrap()
    }

    fn dest(&self, index: usize) -> usize {
        let offsetted = index as i32 + self.value(index);
        (if offsetted < 0 {
            self.len() as i32 - offsetted.abs() - 1
        } else {
            offsetted % self.len() as i32
        }) as usize
    }

    fn move_index(&mut self, source: usize) -> &mut Self {
        if self.source[source] == 0 {
            return self;
        }

        let index = self.index(source);
        let dest = self.dest(index);
        let val = self.index_map.remove(index);
        self.index_map.insert(dest, val);
        self
    }

    fn result(&self) -> Vec<i32> {
        (0..self.len()).map(|i| self.value(i)).collect()
    }
}

fn part01(input: &str) -> i32 {
    unimplemented!()
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(List::parse(EXAMPLE), List::new(vec![1, 2, -3, 3, -2, 0, 4]));
    }

    #[test]
    fn test_dest() {
        assert_eq!(List::new(vec![4, 5, 6, 1, 7, 8, 9]).dest(3), 4);
        assert_eq!(List::new(vec![4, -2, 5, 6, 7, 8, 9]).dest(1), 5);
    }

    #[test]
    fn test_move_index() {
        assert_eq!(
            List::new(vec![4, 5, 6, 1, 7, 8, 9]).move_index(3).result(),
            vec![4, 5, 6, 7, 1, 8, 9]
        );
        assert_eq!(
            List::new(vec![4, -2, 5, 6, 7, 8, 9]).move_index(1).result(),
            vec![4, 5, 6, 7, 8, -2, 9]
        );
        assert_eq!(
            List::new(vec![4, 2, -1, 6, 7, 8, 9]).move_index(2).result(),
            vec![4, -1, 2, 6, 7, 8, 9]
        );

        let mut list = List::parse(EXAMPLE);
        list.move_index(0);
        assert_eq!(list.result(), vec![2, 1, -3, 3, -2, 0, 4]);
        list.move_index(1);
        assert_eq!(list.result(), vec![1, -3, 2, 3, -2, 0, 4]);
        list.move_index(2);
        assert_eq!(list.result(), vec![1, 2, 3, -2, -3, 0, 4]);
        list.move_index(3);
        assert_eq!(list.result(), vec![1, 2, -2, -3, 0, 3, 4]);
        dbg!(list.source[4]);
        list.move_index(4);
        assert_eq!(list.result(), vec![1, 2, -3, 0, 3, 4, -2]);
        list.move_index(5);
        assert_eq!(list.result(), vec![1, 2, -3, 0, 3, 4, -2]);
        list.move_index(6);
        assert_eq!(list.result(), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_mix_once() {
        let mut list = List::parse(EXAMPLE);
        for i in 0..list.len() {
            list.move_index(i);
        }
        assert_eq!(list.value_after_zero(1000), 4);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), -1);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
    }

    const EXAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;
}
