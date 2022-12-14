use std::{cell::RefCell, str::Lines};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/11")));
    println!("02: {}", part02(include_str!("../inputs/11")));
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Item(i64);

impl Item {
    fn inspect(&mut self, operation: &Operation) {
        self.0 = operation.run(self.0);
    }
}

#[derive(Debug, PartialEq)]
enum Value {
    Current,
    Literal(i64),
}

impl Value {
    fn parse(s: &str) -> Self {
        if s == "old" {
            Self::Current
        } else {
            Self::Literal(s.parse().unwrap())
        }
    }

    fn value(&self, current: i64) -> i64 {
        match self {
            Value::Current => current,
            Value::Literal(x) => *x,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Operation {
    lhs: Value,
    rhs: Value,
    op: Operator,
}

impl Operation {
    fn parse(lines: &mut Lines) -> Self {
        let mut comps = lines.next().unwrap().split_whitespace();
        assert_eq!(comps.next(), Some("Operation:"));
        assert_eq!(comps.next(), Some("new"));
        assert_eq!(comps.next(), Some("="));
        let lhs = Value::parse(comps.next().unwrap());
        let op = Operator::parse(comps.next().unwrap());
        let rhs = Value::parse(comps.next().unwrap());
        Self { lhs, rhs, op }
    }

    fn run(&self, current: i64) -> i64 {
        let lhs = self.lhs.value(current);
        let rhs = self.rhs.value(current);
        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Test {
    divisible_by: i64,
    true_target: usize,
    false_target: usize,
}

impl Test {
    fn parse(lines: &mut Lines) -> Self {
        let divisible_by = {
            let mut comps = lines.next().unwrap().split_whitespace();
            assert_eq!(comps.next(), Some("Test:"));
            assert_eq!(comps.next(), Some("divisible"));
            assert_eq!(comps.next(), Some("by"));
            comps.next().unwrap().parse().unwrap()
        };
        let true_target = {
            let mut comps = lines.next().unwrap().split_whitespace();
            assert_eq!(
                (0..5).map(|_| comps.next()).collect::<Option<Vec<_>>>(),
                Some(vec!["If", "true:", "throw", "to", "monkey"])
            );
            comps.next().unwrap().parse().unwrap()
        };
        let false_target = {
            let mut comps = lines.next().unwrap().split_whitespace();
            assert_eq!(
                (0..5).map(|_| comps.next()).collect::<Option<Vec<_>>>(),
                Some(vec!["If", "false:", "throw", "to", "monkey"])
            );
            comps.next().unwrap().parse().unwrap()
        };
        Self {
            divisible_by,
            true_target,
            false_target,
        }
    }

    fn run(&self, item: &Item) -> usize {
        if item.0 % self.divisible_by == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: RefCell<Vec<Item>>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn parse(lines: &mut Lines) -> Self {
        let items = RefCell::new({
            let (a, b) = lines.next().unwrap().split_once(": ").unwrap();
            assert_eq!(a.trim(), "Starting items");
            b.split_terminator(", ")
                .map(|s| Item(s.parse().unwrap()))
                .collect()
        });
        let operation = Operation::parse(lines);
        let test = Test::parse(lines);
        Self {
            items,
            operation,
            test,
        }
    }
}

fn part01(input: &str) -> usize {
    let mut lines = input.lines();
    let mut monkeys = Vec::new();
    while let Some(s) = lines.next() {
        assert!(s.starts_with("Monkey "));
        monkeys.push(Monkey::parse(&mut lines));

        if lines.next().is_none() {
            break;
        }
    }
    let mut inspects = vec![0_usize; monkeys.len()];
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in &mut monkey.items.borrow_mut().iter_mut() {
                item.inspect(&monkey.operation);
                item.0 /= 3;
                inspects[i] += 1;
                let target = monkey.test.run(item);
                monkeys[target].items.borrow_mut().push(*item);
            }
            monkey.items.borrow_mut().clear();
        }
    }
    inspects.sort();
    let mut inspects_iter = inspects.iter();
    inspects_iter.next_back().unwrap() * inspects_iter.next_back().unwrap()
}

fn part02(input: &str) -> usize {
    let mut lines = input.lines();
    let mut monkeys = Vec::new();
    while let Some(s) = lines.next() {
        assert!(s.starts_with("Monkey "));
        monkeys.push(Monkey::parse(&mut lines));

        if lines.next().is_none() {
            break;
        }
    }
    let product = monkeys
        .iter()
        .map(|m| m.test.divisible_by)
        .fold(1, |acc, x| acc * x);
    let mut inspects = vec![0_usize; monkeys.len()];
    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in &mut monkey.items.borrow_mut().iter_mut() {
                item.inspect(&monkey.operation);
                item.0 %= product;
                inspects[i] += 1;
                let target = monkey.test.run(item);
                monkeys[target].items.borrow_mut().push(*item);
            }
            monkey.items.borrow_mut().clear();
        }
    }
    inspects.sort();
    let mut inspects_iter = inspects.iter();
    inspects_iter.next_back().unwrap() * inspects_iter.next_back().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_item() {
        let mut item = Item(79);
        item.inspect(&Operation::parse(&mut "Operation: new = old * 19".lines()));
        assert_eq!(item, Item(1501));

        let mut item = Item(54);
        item.inspect(&Operation::parse(&mut "Operation: new = old + 6".lines()));
        assert_eq!(item, Item(60));
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            Operation::parse(&mut "  Operation: new = old * 19".lines()),
            Operation {
                lhs: Value::Current,
                rhs: Value::Literal(19),
                op: Operator::Multiply
            }
        );
        assert_eq!(
            Operation::parse(&mut "  Operation: new = old + 6".lines()),
            Operation {
                lhs: Value::Current,
                rhs: Value::Literal(6),
                op: Operator::Add
            }
        );
        assert_eq!(
            Operation::parse(&mut "  Operation: new = old * old".lines()),
            Operation {
                lhs: Value::Current,
                rhs: Value::Current,
                op: Operator::Multiply
            }
        );
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(
            Test::parse(
                &mut r#"  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#
                    .lines()
            ),
            Test {
                divisible_by: 23,
                true_target: 2,
                false_target: 3,
            }
        );
    }

    #[test]
    fn test_run_test() {
        let test = Test {
            divisible_by: 23,
            true_target: 2,
            false_target: 3,
        };
        assert_eq!(test.run(&Item(23)), 2);
        assert_eq!(test.run(&Item(500)), 3);
    }

    #[test]
    fn test_parse_monkey() {
        assert_eq!(
            Monkey::parse(
                &mut r#"  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#
                    .lines()
            ),
            Monkey {
                items: RefCell::new(vec![Item(79), Item(98)]),
                operation: Operation {
                    lhs: Value::Current,
                    rhs: Value::Literal(19),
                    op: Operator::Multiply
                },
                test: Test {
                    divisible_by: 23,
                    true_target: 2,
                    false_target: 3
                }
            }
        )
    }

    #[test]
    fn example01() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
        assert_eq!(part01(input), 10605);
    }

    #[test]
    fn example02() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
        assert_eq!(part02(input), 2713310158);
    }
}
