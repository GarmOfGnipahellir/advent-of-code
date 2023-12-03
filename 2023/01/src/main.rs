const INP: &str = include_str!("../in.txt");

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

fn part1(inp: &str) -> i32 {
    let mut sum = 0;
    for line in inp.lines() {
        let mut digits = line.chars().filter(|ch| ch.is_ascii_digit());
        let first = digits.next().unwrap();
        let last = if let Some(last) = digits.last() {
            last
        } else {
            first
        };
        let value = format!("{first}{last}").parse::<i32>().unwrap();
        sum += value;
    }
    sum
}

fn main() {
    println!("Part 1:");
    println!("  EX: {}", part1(EX1));
    println!("  {}", part1(INP));
}
