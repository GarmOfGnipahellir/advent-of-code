use std::ops::{Deref, DerefMut};

const INP: &str = include_str!("../in.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sequence(Vec<i32>);

impl Deref for Sequence {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Sequence {
    fn parse(inp: &str) -> Self {
        Self(
            inp.trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        )
    }

    fn next(&self) -> Option<Self> {
        self.0.iter().any(|&x| x != 0).then_some(Self(
            (0..self.len() - 1).map(|i| self[i + 1] - self[i]).collect(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct History(Vec<Sequence>);

impl Deref for History {
    type Target = Vec<Sequence>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for History {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl History {
    fn from_sequence(seq: Sequence) -> Self {
        let mut seqs = vec![seq.clone()];
        let mut current = seq;
        while let Some(next) = current.next() {
            seqs.push(next.clone());
            current = next;
        }
        Self(seqs)
    }

    fn parse(inp: &str) -> Self {
        Self::from_sequence(Sequence::parse(inp))
    }

    fn extrapolate(&self) -> Self {
        let mut hist = self.clone();

        let mut new = vec![0];
        for i in (0..self.len() - 1).rev() {
            let seq = &self.0[i];
            let prev = seq.0[seq.len() - 1];
            let under = new[new.len() - 1];
            new.push(prev + under);
        }

        for (i, x) in new.into_iter().rev().enumerate() {
            hist.0[i].push(x)
        }

        hist
    }

    fn extrapolate_back(&self) -> Self {
        let mut hist = self.clone();

        let mut new = vec![0];
        for i in (0..self.len() - 1).rev() {
            let seq = &self.0[i];
            let prev = seq.0[0];
            let under = new[new.len() - 1];
            new.push(prev - under);
        }

        for (i, x) in new.into_iter().rev().enumerate() {
            hist.0[i].insert(0, x)
        }

        hist
    }
}

fn part1(inp: &str) -> i32 {
    inp.lines()
        .map(|line| History::parse(line))
        .map(|hist| hist.extrapolate())
        .map(|hist| *hist.0.first().unwrap().last().unwrap())
        .sum()
}

fn part2(inp: &str) -> i32 {
    inp.lines()
        .map(|line| History::parse(line))
        .map(|hist| hist.extrapolate_back())
        .map(|hist| *hist.0.first().unwrap().first().unwrap())
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(INP));
    println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn tes_sequence_parse() {
        assert_eq!(
            Sequence::parse("0 3 6 9 12 15"),
            Sequence(vec![0, 3, 6, 9, 12, 15])
        );
        assert_eq!(
            Sequence::parse("1 3 6 10 15 21"),
            Sequence(vec![1, 3, 6, 10, 15, 21])
        );
        assert_eq!(
            Sequence::parse("10 13 16 21 30 45"),
            Sequence(vec![10, 13, 16, 21, 30, 45])
        );
    }

    #[test]
    fn tes_sequence_next() {
        assert_eq!(
            Sequence::parse("0 3 6 9 12 15").next(),
            Some(Sequence(vec![3, 3, 3, 3, 3]))
        );
        assert_eq!(
            Sequence::parse("3 3 3 3 3").next(),
            Some(Sequence(vec![0, 0, 0, 0]))
        );
        assert_eq!(Sequence::parse("0 0 0 0").next(), None);
    }

    #[test]
    fn tes_history_parse() {
        assert_eq!(
            History::parse("0 3 6 9 12 15"),
            History(vec![
                Sequence::parse("0 3 6 9 12 15"),
                Sequence::parse("3 3 3 3 3"),
                Sequence::parse("0 0 0 0")
            ])
        );
        assert_eq!(
            History::parse("1 3 6 10 15 21"),
            History(vec![
                Sequence::parse("1 3 6 10 15 21"),
                Sequence::parse("2 3 4 5 6"),
                Sequence::parse("1 1 1 1 "),
                Sequence::parse("0 0 0"),
            ])
        );
        assert_eq!(
            History::parse("10 13 16 21 30 45"),
            History(vec![
                Sequence::parse("10 13 16 21 30 45"),
                Sequence::parse("3 3 5 9 15"),
                Sequence::parse("0 2 4 6"),
                Sequence::parse("2 2 2"),
                Sequence::parse("0 0"),
            ])
        );
    }

    #[test]
    fn tes_history_extrapolate() {
        assert_eq!(
            History::parse("0 3 6 9 12 15").extrapolate(),
            History(vec![
                Sequence::parse("0 3 6 9 12 15 18"),
                Sequence::parse("3 3 3 3 3 3"),
                Sequence::parse("0 0 0 0 0")
            ])
        );
        assert_eq!(
            History::parse("1 3 6 10 15 21").extrapolate(),
            History(vec![
                Sequence::parse("1 3 6 10 15 21 28"),
                Sequence::parse("2 3 4 5 6 7"),
                Sequence::parse("1 1 1 1 1"),
                Sequence::parse("0 0 0 0"),
            ])
        );
        assert_eq!(
            History::parse("10 13 16 21 30 45").extrapolate(),
            History(vec![
                Sequence::parse("10 13 16 21 30 45 68"),
                Sequence::parse("3 3 5 9 15 23"),
                Sequence::parse("0 2 4 6 8"),
                Sequence::parse("2 2 2 2"),
                Sequence::parse("0 0 0"),
            ])
        );
    }

    #[test]
    fn tes_history_extrapolate_back() {
        assert_eq!(
            History::parse("10 13 16 21 30 45").extrapolate_back(),
            History(vec![
                Sequence::parse("5 10 13 16 21 30 45"),
                Sequence::parse("5 3 3 5 9 15"),
                Sequence::parse("-2 0 2 4 6"),
                Sequence::parse("2 2 2 2"),
                Sequence::parse("0 0 0"),
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX1), 2);
    }
}
