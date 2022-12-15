use std::{cell::RefCell, cmp::Ordering, collections::VecDeque, fmt::Display, rc::Rc};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/13")));
    println!("02: {}", part02(include_str!("../inputs/13")));
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Single(u32),
    List(Rc<RefCell<Vec<Value>>>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Single(x) => write!(f, "{x}"),
            Value::List(l) => write!(
                f,
                "[{}]",
                l.borrow()
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

impl Value {
    fn compare(&self, other: &Value) -> Ordering {
        match (self, other) {
            (Value::Single(a), Value::Single(b)) => a.cmp(b),
            (Value::Single(a), Value::List(b)) => {
                let a = Value::List(Rc::new(RefCell::new(vec![Value::Single(*a)])));
                let b = Value::List(b.clone());
                a.compare(&b)
            }
            (Value::List(a), Value::Single(b)) => {
                let a = Value::List(a.clone());
                let b = Value::List(Rc::new(RefCell::new(vec![Value::Single(*b)])));
                a.compare(&b)
            }
            (Value::List(a), Value::List(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                let (mut ait, mut bit) = (a.iter(), b.iter());
                loop {
                    match (ait.next(), bit.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.compare(b) {
                            Ordering::Equal => continue,
                            ord => return ord,
                        },
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Packet(Value);

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Packet {
    fn parse(s: &str) -> Self {
        let mut lists: Vec<Rc<RefCell<Vec<Value>>>> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<Vec<Value>>>> = VecDeque::new();
        let mut int = String::new();
        for c in s.chars() {
            match c {
                '[' => {
                    let list = Rc::new(RefCell::new(Vec::new()));
                    lists.push(list.clone());
                    if let Some(back) = queue.back() {
                        back.borrow_mut().push(Value::List(list.clone()));
                    }
                    queue.push_back(list);
                }
                ']' => {
                    if !int.is_empty() {
                        queue
                            .back()
                            .unwrap()
                            .borrow_mut()
                            .push(Value::Single(int.parse().unwrap()));
                        int.clear();
                    }
                    queue.pop_back().unwrap();
                }
                ',' => {
                    if !int.is_empty() {
                        queue
                            .back()
                            .unwrap()
                            .borrow_mut()
                            .push(Value::Single(int.parse().unwrap()));
                        int.clear();
                    }
                }
                c if c.is_ascii_digit() => {
                    int.push(c);
                }
                _ => unreachable!(),
            }
        }

        Self(Value::List(lists[0].clone()))
    }

    fn compare(&self, other: &Packet) -> Ordering {
        self.0.compare(&other.0)
    }
}

fn part01(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut index = 0;
    let mut sum = 0;
    while let (Some(a), Some(b)) = (lines.next(), lines.next()) {
        index += 1;

        let a = Packet::parse(a);
        let b = Packet::parse(b);
        let ord = a.compare(&b);

        if ord.is_le() {
            sum += index;
        }

        if lines.next().is_none() {
            break;
        }
    }
    sum
}

fn part02(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter_map(|line| if line.is_empty() { None } else { Some(line) })
        .map(|line| Packet::parse(line))
        .collect::<Vec<_>>();
    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));

    packets.sort_by(|a, b| a.compare(b));

    let mut res = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &Packet::parse("[[2]]") || packet == &Packet::parse("[[6]]") {
            res *= i + 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! val {
        ($n: literal) => {
            Value::Single($n)
        };
        [$($n: expr),*] => {
            Value::List(Rc::new(RefCell::new(vec![$($n),*])))
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(Packet::parse("[1]"), Packet(val![val!(1)]));
        assert_eq!(Packet::parse("[1,2]"), Packet(val![val!(1), val!(2)]));
        assert_eq!(
            Packet::parse("[[1,2]]"),
            Packet(val![val![val!(1), val!(2)]])
        );
        assert_eq!(
            Packet::parse("[[1,2],[3]]"),
            Packet(val![val![val!(1), val!(2)], val![val!(3)]])
        );
        assert_eq!(Packet::parse("[10]"), Packet(val![val!(10)]));
    }

    #[test]
    fn test_compare() {
        assert_eq!(val!(1).compare(&val!(2)), Ordering::Less);
        assert_eq!(val!(2).compare(&val!(1)), Ordering::Greater);
        assert_eq!(val!(1).compare(&val!(1)), Ordering::Equal);

        assert_eq!(
            val![val!(2), val!(3), val!(4)].compare(&val!(4)),
            Ordering::Less
        );

        assert_eq!(
            val!(9).compare(&val![val!(8), val!(7), val!(6)]),
            Ordering::Greater
        );
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 13);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 140);
    }

    const EXAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
}
