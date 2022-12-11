fn main() {
    println!("01: {}", part01(include_str!("../inputs/01")));
    println!("02: {}", part02(include_str!("../inputs/01")));
}

fn part01(input: &str) -> i32 {
    let mut elfs = Vec::new();
    let mut elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elfs.push(elf);
            elf = 0;
            continue;
        }

        elf += line.parse::<i32>().unwrap();
    }
    let mut max_elf = -1;
    for elf in elfs {
        if elf > max_elf {
            max_elf = elf
        }
    }
    return max_elf;
}

fn part02(input: &str) -> i32 {
    let mut elfs = Vec::new();
    let mut elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elfs.push(elf);
            elf = 0;
            continue;
        }

        elf += line.parse::<i32>().unwrap();
    }

    elfs.sort();

    let num = elfs.len();
    return elfs[num - 1] + elfs[num - 2] + elfs[num - 3];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;
        assert_eq!(part01(input), 24000);
    }

    #[test]
    fn example02() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;
        assert_eq!(part02(input), 45000);
    }
}
