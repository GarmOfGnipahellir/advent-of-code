fn main() {
    println!("01: {}", part01(include_str!("../inputs/01")));
    println!("02: {}", part02(include_str!("../inputs/01")));
}

struct Packet(Vec<i32>);

fn part01(input: &str) -> usize {
    unimplemented!()
}

fn part02(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 13);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 13);
    }

    const EXAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
}
