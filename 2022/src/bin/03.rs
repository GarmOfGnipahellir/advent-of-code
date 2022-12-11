fn main() {
    println!("01: {}", part01(include_str!("../inputs/03")));
    println!("02: {}", part02(include_str!("../inputs/03")));
}

fn get_prio(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u32 - 96) as i32
    } else {
        (c as u32 - 38) as i32
    }
}

fn part01(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut errs = Vec::new();
        let (a, b) = line.split_at(line.len() / 2);
        for c1 in a.chars() {
            for c2 in b.chars() {
                if c1 == c2 {
                    if !errs.contains(&c1) {
                        errs.push(c1);
                    }
                }
            }
        }
        for c in errs {
            sum += get_prio(c);
        }
    }
    return sum;
}

fn part02(input: &str) -> i32 {
    let mut sum = 0;
    for chunk in input.lines().collect::<Vec<_>>().chunks(3) {
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                sum += get_prio(c);
                break;
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prio() {
        assert_eq!(get_prio('a'), 1);
        assert_eq!(get_prio('z'), 26);
        assert_eq!(get_prio('A'), 27);
        assert_eq!(get_prio('Z'), 52);
    }

    #[test]
    fn example01() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part01(input), 157);
    }

    #[test]
    fn example02() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part02(input), 70);
    }
}
