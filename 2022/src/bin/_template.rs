fn main() {
    println!("01: {}", part01(include_str!("../inputs/01")));
    println!("02: {}", part02(include_str!("../inputs/01")));
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
    fn example01() {
        let input = r#""#;
        assert_eq!(part01(input), -1);
    }

    #[test]
    fn example02() {
        let input = r#""#;
        assert_eq!(part02(input), -1);
    }
}
