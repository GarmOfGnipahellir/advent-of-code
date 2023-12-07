use std::{cmp::Ordering, collections::HashMap};

const INP: &str = include_str!("../in.txt");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Label2,
    Label3,
    Label4,
    Label5,
    Label6,
    Label7,
    Label8,
    Label9,
    LabelT,
    LabelJ,
    LabelQ,
    LabelK,
    LabelA,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                unreachable!()
            }
            ord => ord,
        }
    }
}

impl Hand {
    fn parse(inp: &str) -> Self {
        let (cards, bid) = inp.split_once(" ").unwrap();
        let cards = {
            let mut chars = cards.chars();
            let chars = [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ];
            chars.map(|ch| match ch {
                '2' => Card::Label2,
                '3' => Card::Label3,
                '4' => Card::Label4,
                '5' => Card::Label5,
                '6' => Card::Label6,
                '7' => Card::Label7,
                '8' => Card::Label8,
                '9' => Card::Label9,
                'T' => Card::LabelT,
                'J' => Card::LabelJ,
                'Q' => Card::LabelQ,
                'K' => Card::LabelK,
                'A' => Card::LabelA,
                _ => unreachable!(),
            })
        };
        let bid = bid.parse::<usize>().unwrap();
        Self { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut counts = HashMap::new();
        for card in &self.cards {
            if let Some(n) = counts.get_mut(card) {
                *n += 1;
            } else {
                counts.insert(card.clone(), 1_usize);
            }
        }
        let counts = counts.into_iter().collect::<Vec<_>>();

        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if counts[0].1 == 4 || counts[1].1 == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if counts[0].1 == 3 || counts[1].1 == 3 || counts[2].1 == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

fn parse_hands(inp: &str) -> Vec<Hand> {
    inp.trim().lines().map(|line| Hand::parse(line)).collect()
}

fn part1(inp: &str) -> usize {
    let mut hands = parse_hands(inp);
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1, h))
        .map(|(i, h)| h.bid * i)
        .sum()
}

fn part2(inp: &str) -> usize {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(INP));
    println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_card_ord() {
        assert!(Card::LabelA > Card::LabelK);
        assert!(Card::LabelK > Card::LabelQ);
        assert!(Card::LabelQ > Card::LabelJ);
        assert!(Card::LabelJ > Card::LabelT);
        assert!(Card::LabelT > Card::Label9);
        assert!(Card::Label9 > Card::Label8);
        assert!(Card::Label8 > Card::Label7);
        assert!(Card::Label7 > Card::Label6);
        assert!(Card::Label6 > Card::Label5);
        assert!(Card::Label5 > Card::Label4);
        assert!(Card::Label4 > Card::Label3);
        assert!(Card::Label3 > Card::Label2);
    }

    #[test]
    fn test_hand_type_ord() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_hand_ord() {
        assert!(Hand::parse("33332 0") > Hand::parse("2AAAA 0"));
        assert!(Hand::parse("77888 0") > Hand::parse("77788 0"));

        assert!(Hand::parse("QQQJA 483") > Hand::parse("T55J5 684"));
        assert!(Hand::parse("T55J5 684") < Hand::parse("QQQJA 483"));

        assert!(Hand::parse("T55J5 684") > Hand::parse("KK677 28"));
        assert!(Hand::parse("KK677 28") < Hand::parse("T55J5 684"));

        assert!(Hand::parse("KK677 28") > Hand::parse("KTJJT 220"));
        assert!(Hand::parse("KTJJT 220") < Hand::parse("KK677 28"));

        assert!(Hand::parse("KTJJT 220") > Hand::parse("32T3K 765"));
        assert!(Hand::parse("32T3K 765") < Hand::parse("KTJJT 220"));
    }

    #[test]
    fn test_hand_parse() {
        assert_eq!(
            Hand::parse("32T3K 765"),
            Hand {
                cards: [
                    Card::Label3,
                    Card::Label2,
                    Card::LabelT,
                    Card::Label3,
                    Card::LabelK
                ],
                bid: 765,
            }
        );
    }

    #[test]
    fn test_hand_hand_type() {
        assert_eq!(Hand::parse("AAAAA 0").hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::parse("AA8AA 0").hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::parse("23332 0").hand_type(), HandType::FullHouse);
        assert_eq!(Hand::parse("TTT98 0").hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::parse("23432 0").hand_type(), HandType::TwoPair);
        assert_eq!(Hand::parse("A23A4 0").hand_type(), HandType::OnePair);
        assert_eq!(Hand::parse("23456 0").hand_type(), HandType::HighCard);

        assert_eq!(Hand::parse("32T3K 765").hand_type(), HandType::OnePair);
        assert_eq!(Hand::parse("T55J5 684").hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::parse("KK677 28").hand_type(), HandType::TwoPair);
        assert_eq!(Hand::parse("KTJJT 220").hand_type(), HandType::TwoPair);
        assert_eq!(Hand::parse("QQQJA 483").hand_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_parse_hands() {
        assert_eq!(
            parse_hands(EX1),
            vec![
                Hand::parse("32T3K 765"),
                Hand::parse("T55J5 684"),
                Hand::parse("KK677 28"),
                Hand::parse("KTJJT 220"),
                Hand::parse("QQQJA 483"),
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 6440);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(EX1), 0);
    }
}
