use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../inputs/21");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("can't parse '{s}'"),
        }
    }

    fn calc(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Monkey<'a> {
    Literal(i64),
    Operation { lhs: &'a str, rhs: &'a str, op: Op },
}

#[derive(Debug)]
struct Monkeys<'a>(HashMap<&'a str, Monkey<'a>>);

impl<'a> Monkeys<'a> {
    fn parse(s: &'a str) -> Self {
        let re = Regex::new(r"(\w{4}): (?:(\d+)|(?:(\w{4}) (.) (\w{4})))").unwrap();
        let mut monkeys = HashMap::new();
        for l in s.lines() {
            let caps = re.captures(l).unwrap();
            let id = caps.get(1).unwrap().as_str();
            let monkey = if let Some(s) = caps.get(2) {
                Monkey::Literal(s.as_str().parse().unwrap())
            } else {
                Monkey::Operation {
                    lhs: caps.get(3).unwrap().as_str(),
                    rhs: caps.get(5).unwrap().as_str(),
                    op: Op::parse(caps.get(4).unwrap().as_str()),
                }
            };
            monkeys.insert(id, monkey);
        }
        Self(monkeys)
    }

    fn resolve(&self, id: &str) -> i64 {
        let monkey = self.0.get(id).unwrap();
        match monkey {
            Monkey::Literal(x) => *x,
            Monkey::Operation { lhs, rhs, op } => op.calc(self.resolve(lhs), self.resolve(rhs)),
        }
    }
}

fn part01(input: &str) -> i64 {
    Monkeys::parse(input).resolve("root")
}

fn part02(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let monkeys = Monkeys::parse(EXAMPLE);
        assert_eq!(
            monkeys.0.get("root"),
            Some(&Monkey::Operation {
                lhs: "pppw",
                rhs: "sjmn",
                op: Op::Add
            })
        );
        assert_eq!(monkeys.0.get("dbpl"), Some(&Monkey::Literal(5)));
        assert_eq!(
            monkeys.0.get("sjmn"),
            Some(&Monkey::Operation {
                lhs: "drzm",
                rhs: "dbpl",
                op: Op::Mul
            })
        );
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 152);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 301);
    }

    const EXAMPLE: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;
}
