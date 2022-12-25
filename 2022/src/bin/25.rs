const INPUT: &str = include_str!("../inputs/25");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

fn snafu_to_decimal(s: &str) -> i32 {
    let len = s.len();
    s.chars().enumerate().fold(0, |acc, (i, c)| {
        let base = 5_i32;
        let power = (len - i - 1) as i32;
        let factor = match c {
            '-' => -1,
            '=' => -2,
            _ => c.to_digit(10).unwrap() as i32,
        };
        acc + (base.pow(power as u32) * factor)
    })
}

fn decimal_to_snafu(x: i32) -> String {
    let base = 5_i32;
    let len = {
        let mut i = 0;
        loop {
            if base.pow(i as u32) >= x {
                break i - 1;
            }
            i += 1;
        }
    };
    println!("{x} len {len}");
    todo!()
}

fn part01(input: &str) -> String {
    let ans: i32 = input.lines().map(|line| snafu_to_decimal(line)).sum();
    decimal_to_snafu(ans)
}

fn part02(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1"), 1);
        assert_eq!(snafu_to_decimal("2"), 2);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("1-"), 4);
        assert_eq!(snafu_to_decimal("10"), 5);
        assert_eq!(snafu_to_decimal("11"), 6);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("2="), 8);
        assert_eq!(snafu_to_decimal("2-"), 9);
        assert_eq!(snafu_to_decimal("20"), 10);
        assert_eq!(snafu_to_decimal("1=0"), 15);
        assert_eq!(snafu_to_decimal("1-0"), 20);
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(snafu_to_decimal("1-0---0"), 12345);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), "2=-1=0".to_string());
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
    }

    const EXAMPLE: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
}
