use std::collections::HashMap;

use regex::Regex;

const INP: &str = include_str!("../in.txt");

const ORDER: &[&str] = &[
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

#[derive(Debug, PartialEq, Eq)]
struct CategoryRange {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl CategoryRange {
    fn parse(inp: &str) -> Self {
        let mut iter = inp
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        Self {
            dest_start: iter.next().unwrap(),
            source_start: iter.next().unwrap(),
            length: iter.next().unwrap(),
        }
    }

    fn contains(&self, id: usize) -> bool {
        id >= self.source_start && id < self.source_start + self.length
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CategoryMap {
    ranges: Vec<CategoryRange>,
}

impl CategoryMap {
    fn get(&self, id: usize) -> usize {
        for range in &self.ranges {
            if range.contains(id) {
                let delta = id - range.source_start;
                return range.dest_start + delta;
            }
        }
        id
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Almenac {
    seeds: Vec<usize>,
    maps: HashMap<(String, String), CategoryMap>,
}

impl Almenac {
    fn parse(inp: &str) -> Self {
        let mut lines = inp.lines();

        let seeds = lines
            .next()
            .unwrap()
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let re = Regex::new(r"(?<source>\w+)-to-(?<dest>\w+) map:").unwrap();
        let mut maps = HashMap::new();

        lines.next().unwrap();

        let mut key = {
            let caps = re.captures(lines.next().unwrap()).unwrap();
            (caps["source"].to_string(), caps["dest"].to_string())
        };
        let mut val = CategoryMap { ranges: vec![] };

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(caps) = re.captures(line) {
                maps.insert(key, val);

                key = (caps["source"].to_string(), caps["dest"].to_string());
                val = CategoryMap { ranges: vec![] };
                continue;
            }

            val.ranges.push(CategoryRange::parse(line));
        }
        maps.insert(key, val);

        Self { seeds, maps }
    }

    fn get(&self, source_id: usize, source_cat: &str, dest_cat: &str) -> usize {
        let mut id = source_id;
        let mut from = source_cat;
        loop {
            let to = ORDER
                .iter()
                .enumerate()
                .find_map(|(i, &x)| if x == from { Some(ORDER[i + 1]) } else { None })
                .unwrap();

            id = self
                .maps
                .get(&(from.to_string(), to.to_string()))
                .unwrap()
                .get(id);

            if to == dest_cat {
                break;
            }

            from = to;
        }
        id
    }
}

fn part1(inp: &str) -> usize {
    let almenac = Almenac::parse(inp);
    almenac
        .seeds
        .iter()
        .copied()
        .map(|x| almenac.get(x, "seed", "location"))
        .min()
        .unwrap()
}

fn part2(inp: &str) -> usize {
    let almenac = Almenac::parse(inp);
    almenac
        .seeds
        .chunks(2)
        .flat_map(|x| x[0]..x[0] + x[1])
        .map(|x| almenac.get(x, "seed", "location"))
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(INP));
    println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_category_range_parse() {
        assert_eq!(
            CategoryRange::parse("50 98 2"),
            CategoryRange {
                dest_start: 50,
                source_start: 98,
                length: 2
            }
        )
    }

    #[test]
    fn test_category_map_get() {
        let map = CategoryMap {
            ranges: vec![
                CategoryRange::parse("50 98 2"),
                CategoryRange::parse("52 50 48"),
            ],
        };
        assert_eq!(map.get(0), 0);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(48), 48);
        assert_eq!(map.get(49), 49);
        assert_eq!(map.get(50), 52);
        assert_eq!(map.get(51), 53);
        assert_eq!(map.get(96), 98);
        assert_eq!(map.get(97), 99);
        assert_eq!(map.get(98), 50);
        assert_eq!(map.get(99), 51);

        assert_eq!(map.get(79), 81);
        assert_eq!(map.get(14), 14);
        assert_eq!(map.get(55), 57);
        assert_eq!(map.get(13), 13);
    }

    #[test]
    fn test_almenac_parse() {
        let mut maps = HashMap::new();
        maps.insert(
            ("seed".to_string(), "soil".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("50 98 2"),
                    CategoryRange::parse("52 50 48"),
                ],
            },
        );
        maps.insert(
            ("soil".to_string(), "fertilizer".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("0 15 37"),
                    CategoryRange::parse("37 52 2"),
                    CategoryRange::parse("39 0 15"),
                ],
            },
        );
        maps.insert(
            ("fertilizer".to_string(), "water".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("49 53 8"),
                    CategoryRange::parse("0 11 42"),
                    CategoryRange::parse("42 0 7"),
                    CategoryRange::parse("57 7 4"),
                ],
            },
        );
        maps.insert(
            ("water".to_string(), "light".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("88 18 7"),
                    CategoryRange::parse("18 25 70"),
                ],
            },
        );
        maps.insert(
            ("light".to_string(), "temperature".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("45 77 23"),
                    CategoryRange::parse("81 45 19"),
                    CategoryRange::parse("68 64 13"),
                ],
            },
        );
        maps.insert(
            ("temperature".to_string(), "humidity".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("0 69 1"),
                    CategoryRange::parse("1 0 69"),
                ],
            },
        );
        maps.insert(
            ("humidity".to_string(), "location".to_string()),
            CategoryMap {
                ranges: vec![
                    CategoryRange::parse("60 56 37"),
                    CategoryRange::parse("56 93 4"),
                ],
            },
        );
        let almenac = Almenac::parse(EX1);
        for (k, v) in maps {
            assert_eq!(almenac.maps.get(&k), Some(&v));
        }
    }

    #[test]
    fn test_almenac_get() {
        let almenac = Almenac::parse(EX1);
        assert_eq!(almenac.get(79, "seed", "location"), 82);
        assert_eq!(almenac.get(14, "seed", "location"), 43);
        assert_eq!(almenac.get(55, "seed", "location"), 86);
        assert_eq!(almenac.get(13, "seed", "location"), 35);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX1), 46);
    }
}
