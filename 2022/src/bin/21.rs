use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use regex::Regex;

const INPUT: &str = include_str!("../inputs/21");

fn main() {
    println!("01: {}", part01(INPUT));
    println!("02: {}", part02(INPUT));
}

#[derive(Debug, PartialEq, Clone)]
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

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Monkey<'a> {
    Literal(i64),
    Operation { lhs: &'a str, rhs: &'a str, op: Op },
}

#[derive(Debug)]
struct Monkeys<'a>(RefCell<HashMap<&'a str, Monkey<'a>>>);

impl<'a> Monkeys<'a> {
    fn new() -> Self {
        Self(RefCell::new(HashMap::new()))
    }

    fn add(&'a self, id: &'a str, monkey: Monkey<'a>) {
        self.0.borrow_mut().insert(id, monkey);
        println!("{self:?}");
    }

    fn get(&'a self, id: &'a str) -> Option<Monkey<'a>> {
        self.0.borrow().get(id).cloned()
    }

    fn parse(s: &'a str) -> Self {
        let re = Regex::new(r"(\w{4}): (?:(\d+)|(?:(\w{4}) (.) (\w{4})))").unwrap();
        let mut res = HashMap::new();
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
            res.insert(id, monkey);
        }
        Self(RefCell::new(res))
    }

    fn resolve(&'a self, id: &'a str) -> i64 {
        let monkey = self.get(id).unwrap();
        match monkey {
            Monkey::Literal(x) => x,
            Monkey::Operation { lhs, rhs, op } => op.calc(self.resolve(lhs), self.resolve(rhs)),
        }
    }

    fn equation(&'a self, id: &'a str) -> String {
        if id == "humn" {
            return id.to_string();
        }

        let monkey = self.get(id).unwrap();
        match monkey {
            Monkey::Literal(x) => x.to_string(),
            Monkey::Operation { lhs, rhs, op } => {
                format!("({} {} {})", self.equation(lhs), op, self.equation(rhs))
            }
        }
    }

    fn simplify(&'a self, id: &'a str) -> (&str, Monkey) {
        let monkey = self.get(id).unwrap();
        match monkey {
            Monkey::Literal(x) => (id, Monkey::Literal(x)),
            Monkey::Operation { lhs, rhs, op } => {
                if lhs == "humn" {
                    let (rid, _) = self.simplify(rhs);
                    return (
                        id,
                        Monkey::Operation {
                            lhs: "humn",
                            rhs: rid,
                            op: op.clone(),
                        },
                    );
                }
                if rhs == "humn" {
                    let (lid, _) = self.simplify(lhs);
                    return (
                        id,
                        Monkey::Operation {
                            lhs: lid,
                            rhs: "humn",
                            op: op.clone(),
                        },
                    );
                }

                let (lid, lhs) = self.simplify(lhs);
                let (rid, rhs) = self.simplify(rhs);
                if let (Monkey::Literal(lhs), Monkey::Literal(rhs)) = (&lhs, &rhs) {
                    let mut self_mut = self.0.borrow_mut();
                    self_mut.remove(lid);
                    self_mut.remove(rid);
                    let new = Monkey::Literal(op.calc(*lhs, *rhs));
                    self_mut.insert(id, new.clone());
                    (id, new)
                } else {
                    (
                        id,
                        Monkey::Operation {
                            lhs: lid,
                            rhs: rid,
                            op: op.clone(),
                        },
                    )
                }
            }
        }
    }
}

fn part01(input: &str) -> i64 {
    Monkeys::parse(input).resolve("root")
}

fn part02(input: &str) -> i64 {
    let monkeys = Monkeys::parse(input);
    if let Monkey::Operation { lhs, rhs, .. } = monkeys.get("root").unwrap() {
        println!("{} = {}", monkeys.equation(lhs), monkeys.equation(rhs));
        monkeys.simplify(lhs);
        monkeys.simplify(rhs);
        println!("{:#?}", monkeys);
        if let Monkey::Operation { lhs, rhs, .. } = monkeys.get("root").unwrap() {
            println!("{} = {}", monkeys.equation(lhs), monkeys.equation(rhs));
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let monkeys = Monkeys::parse(EXAMPLE);
        assert_eq!(
            monkeys.get("root"),
            Some(Monkey::Operation {
                lhs: "pppw",
                rhs: "sjmn",
                op: Op::Add
            })
        );
        assert_eq!(monkeys.get("dbpl"), Some(Monkey::Literal(5)));
        assert_eq!(
            monkeys.get("sjmn"),
            Some(Monkey::Operation {
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
        part02(EXAMPLE);
        // assert_eq!(part02(EXAMPLE), 301);
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
