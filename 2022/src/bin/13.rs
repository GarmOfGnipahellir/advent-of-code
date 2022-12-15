use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/01")));
    println!("02: {}", part02(include_str!("../inputs/01")));
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Single(u32),
    List(Rc<RefCell<Vec<Value>>>),
}

#[derive(Debug, PartialEq)]
struct Packet(Value);

impl Packet {
    fn parse(s: &str) -> Self {
        let mut lists: Vec<Rc<RefCell<Vec<Value>>>> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<Vec<Value>>>> = VecDeque::new();
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
                    queue.pop_back().unwrap();
                }
                ',' => {}
                c if c.is_digit(10) => {
                    queue
                        .back()
                        .unwrap()
                        .borrow_mut()
                        .push(Value::Single(c.to_digit(10).unwrap()));
                }
                _ => unreachable!(),
            }
            println!("{:?}", lists);
        }
        println!("");

        // let res = lists[0].borrow().clone();
        Self(Value::List(lists[0].clone()))
    }
}

fn part01(input: &str) -> usize {
    unimplemented!()
}

fn part02(input: &str) -> usize {
    unimplemented!()
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
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 13);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), 13);
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
