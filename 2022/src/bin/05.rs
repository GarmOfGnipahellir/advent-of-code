use std::collections::VecDeque;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/01")));
    println!("02: {}", part02(include_str!("../inputs/01")));
}

fn part01(input: &str) -> &str {
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
        }
    }
    println!("{:#?}", stacks);
    ""
}

fn part02(input: &str) -> &str {
    ""
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
        assert_eq!(part01(input), "");
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
        assert_eq!(part02(input), "");
    }
}
