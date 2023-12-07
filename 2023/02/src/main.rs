const INP: &str = include_str!("../in.txt");

fn part1(inp: &str) -> i32 {
    todo!()
}

fn part2(inp: &str) -> i32 {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(INP));
    println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const EX2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX2), 281);
    }
}
