const INP: &str = include_str!("../in.txt");

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

fn parse_digit(inp: &str, start: usize) -> Option<char> {
    let chars = inp.chars().collect::<Vec<_>>();
    if chars[start].is_ascii_digit() {
        return Some(chars[start]);
    }

    let mut buffer = String::new();
    for i in start..chars.len() {
        if chars[i].is_ascii_digit() {
            break;
        }

        buffer.push(chars[i]);

        match buffer.as_str() {
            "one" => return Some('1'),
            "two" => return Some('2'),
            "three" => return Some('3'),
            "four" => return Some('4'),
            "five" => return Some('5'),
            "six" => return Some('6'),
            "seven" => return Some('7'),
            "eight" => return Some('8'),
            "nine" => return Some('9'),
            _ => continue,
        }
    }

    None
}

fn part2(inp: &str) -> i32 {
    let mut sum = 0;
    for line in inp.lines() {
        let mut digits = (0..line.len()).filter_map(|i| parse_digit(line, i));

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
