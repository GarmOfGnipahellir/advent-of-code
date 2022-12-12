fn main() {
    println!("01: {}", part01(include_str!("../inputs/04")));
    println!("02: {}", part02(include_str!("../inputs/04")));
}

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn parse(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        Self::new(start.parse().unwrap(), end.parse().unwrap())
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

fn part01(input: &str) -> i32 {
    let mut n = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let (a, b) = (Range::parse(a), Range::parse(b));
        if a.contains(&b) || b.contains(&a) {
            n += 1
        }
    }
    n
}

fn part02(input: &str) -> i32 {
    let mut n = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let (a, b) = (Range::parse(a), Range::parse(b));
        if a.overlaps(&b) || b.overlaps(&a) {
            n += 1
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_parse() {
        assert_eq!(Range::parse("2-3"), Range::new(2, 3));
        assert_eq!(Range::parse("13-19"), Range::new(13, 19));
    }

    #[test]
    fn test_range_contains() {
        assert!(Range::new(2, 8).contains(&Range::new(3, 7)));
        assert!(Range::new(4, 6).contains(&Range::new(6, 6)));
    }

    #[test]
    fn test_range_overlaps() {
        assert!(Range::new(5, 7).overlaps(&Range::new(7, 9)));
        assert!(Range::new(2, 8).overlaps(&Range::new(3, 7)));
    }

    #[test]
    fn example01() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(part01(input), 2);
    }

    #[test]
    fn example02() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(part02(input), 4);
    }
}
