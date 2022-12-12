use std::collections::VecDeque;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/05")));
    println!("02: {}", part02(include_str!("../inputs/05")));
}

fn part01(input: &str) -> String {
    let mut stacks = Vec::new();
    for line in input.lines() {
        if line.contains('[') {
            for (i, chunk) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
                let s = chunk.iter().collect::<String>();
                let s = s.trim();
                if stacks.len() <= i {
                    stacks.push(VecDeque::new());
                }
                if !s.is_empty() {
                    stacks[i].push_front(s.chars().collect::<Vec<_>>()[1]);
                }
            }
        } else if line.contains("move") {
            let mut iter = line.split_whitespace();
            assert_eq!(iter.next(), Some("move"));
            let num = iter.next().unwrap().parse::<usize>().unwrap();
            assert_eq!(iter.next(), Some("from"));
            let from = iter.next().unwrap().parse::<usize>().unwrap();
            assert_eq!(iter.next(), Some("to"));
            let to = iter.next().unwrap().parse::<usize>().unwrap();

            for _ in 0..num {
                let c = stacks[from - 1].pop_back().unwrap();
                stacks[to - 1].push_back(c);
            }
        }
    }
    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

fn part02(input: &str) -> String {
    let mut stacks = Vec::new();
    for line in input.lines() {
        if line.contains('[') {
            for (i, chunk) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
                let s = chunk.iter().collect::<String>();
                let s = s.trim();
                if stacks.len() <= i {
                    stacks.push(VecDeque::new());
                }
                if !s.is_empty() {
                    stacks[i].push_front(s.chars().collect::<Vec<_>>()[1]);
                }
            }
        } else if line.contains("move") {
            let mut iter = line.split_whitespace();
            assert_eq!(iter.next(), Some("move"));
            let num = iter.next().unwrap().parse::<usize>().unwrap();
            assert_eq!(iter.next(), Some("from"));
            let from = iter.next().unwrap().parse::<usize>().unwrap();
            assert_eq!(iter.next(), Some("to"));
            let to = iter.next().unwrap().parse::<usize>().unwrap();

            let len = stacks[from - 1].len();
            let mut end = stacks[from - 1].split_off(len - num);
            stacks[to - 1].append(&mut end);
        }
    }
    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        assert_eq!(part01(input), "CMZ");
    }

    #[test]
    fn example02() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        assert_eq!(part02(input), "MCD");
    }
}
