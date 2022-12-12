use std::collections::VecDeque;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/06")));
    println!("02: {}", part02(include_str!("../inputs/06")));
}

fn part01(input: &str) -> usize {
    let mut buf = VecDeque::<char>::new();
    for (i, c) in input.chars().enumerate() {
        if buf.len() == 4 {
            let mut checked = Vec::<char>::new();
            let mut is_marker = true;
            for &c in &buf {
                if checked.contains(&c) {
                    is_marker = false;
                    break;
                }
                checked.push(c);
            }

            if is_marker {
                return i;
            }

            buf.pop_front().unwrap();
        }
        buf.push_back(c);
    }
    0
}

fn part02(input: &str) -> usize {
    let mut buf = VecDeque::<char>::new();
    for (i, c) in input.chars().enumerate() {
        if buf.len() == 14 {
            let mut checked = Vec::<char>::new();
            let mut is_marker = true;
            for &c in &buf {
                if checked.contains(&c) {
                    is_marker = false;
                    break;
                }
                checked.push(c);
            }

            if is_marker {
                return i;
            }

            buf.pop_front().unwrap();
        }
        buf.push_back(c);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(part01("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part01("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part01("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part01("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part01("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn example02() {
        assert_eq!(part02("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part02("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part02("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part02("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part02("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
