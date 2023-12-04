const INP: &str = include_str!("../in.txt");

fn part1(inp: &str) -> i32 {
    inp.lines()
        .map(|line| {
            let (head, tail) = line.split_once(":").unwrap();
            let id = head.replace("Card ", "").trim().parse::<i32>().unwrap();

            let (head, tail) = tail.split_once("|").unwrap();
            let card_nums = head
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let win_nums = tail
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (id, card_nums, win_nums)
        })
        .map(|(_, card, win)| {
            // 2^(n - 1)
            let n = card.iter().filter(|&x| win.contains(x)).count() as u32;
            if n > 0 {
                2_i32.pow(n - 1)
            } else {
                0
            }
        })
        .sum::<i32>()
}

fn main() {
    println!("Part 1: {}", part1(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 13);
    }
}
