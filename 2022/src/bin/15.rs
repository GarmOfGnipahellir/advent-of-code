use std::collections::HashSet;

use glam::{ivec2, IVec2};
use rayon::prelude::*;
use regex::Regex;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/15"), 2000000));
    println!("02: {}", part02(include_str!("../inputs/15"), 4000000));
}

#[derive(Debug, PartialEq)]
struct Sensor {
    position: IVec2,
    closest_beacon: IVec2,
}

impl Sensor {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^Sensor at x=(.+?), y=(.+?): closest beacon is at x=(.+?), y=(.+?)$")
            .unwrap();
        let caps = re.captures(s).unwrap();

        Self {
            position: ivec2(
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            ),
            closest_beacon: ivec2(
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
            ),
        }
    }

    fn perimeter(&self) -> HashSet<IVec2> {
        let closest_delta = self.closest_beacon - self.position;
        let closest_dist = closest_delta.x.abs() + closest_delta.y.abs() + 1;

        let mut res = HashSet::new();
        for y in -closest_dist..=closest_dist {
            let t = (closest_dist + 1) - y.abs() - 1;
            res.insert(ivec2(-t, y) + self.position);
            res.insert(ivec2(t, y) + self.position);
        }
        res
    }
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<IVec2>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let sensors = s
            .lines()
            .map(|line| Sensor::parse(line))
            .collect::<Vec<_>>();
        let beacons = sensors.iter().map(|sensor| sensor.closest_beacon).collect();
        Self { sensors, beacons }
    }

    fn min_max(&self) -> (IVec2, IVec2) {
        let (mut min, mut max) = (IVec2::ONE * i32::MAX, IVec2::ONE * i32::MIN);
        let positions = self
            .sensors
            .iter()
            .map(|s| s.position)
            .chain(self.beacons.iter().map(|p| *p))
            .collect::<Vec<_>>();
        for p in positions {
            min.x = i32::min(p.x, min.x);
            min.y = i32::min(p.y, min.y);
            max.x = i32::max(p.x, max.x);
            max.y = i32::max(p.y, max.y);
        }
        (min, max)
    }

    fn must_be_empty(&self, pos: IVec2) -> bool {
        if self.beacons.contains(&pos) {
            return false;
        }

        for sensor in &self.sensors {
            let closest_delta = sensor.closest_beacon - sensor.position;
            let closest_dist = closest_delta.x.abs() + closest_delta.y.abs();

            let delta = pos - sensor.position;
            let dist = delta.x.abs() + delta.y.abs();
            if dist <= closest_dist {
                return true;
            }
        }

        false
    }

    fn can_be_beacon(&self, pos: IVec2) -> bool {
        !self.must_be_empty(pos)
            && self
                .sensors
                .iter()
                .find(|&s| s.position == pos || s.closest_beacon == pos)
                .is_none()
    }
}

// could be optimized further with sensor perimiters
// kinda like part 02
fn part01(input: &str, row: i32) -> i32 {
    let map = Map::parse(input);
    let (min, max) = map.min_max();
    (min.x * 10..=max.x * 10)
        .into_par_iter()
        .map(|x| {
            if map.must_be_empty(ivec2(x, row)) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part02(input: &str, limit: i32) -> usize {
    let map = Map::parse(input);

    let p = map
        .sensors
        .par_iter()
        .flat_map(|s| s.perimeter())
        .filter(|p| p.x >= 0 && p.x <= limit && p.y >= 0 && p.y <= limit)
        .find_any(|p| map.can_be_beacon(*p))
        .unwrap();

    p.x as usize * 4000000 + p.y as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sensor() {
        assert_eq!(
            Sensor::parse("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Sensor {
                position: ivec2(2, 18),
                closest_beacon: ivec2(-2, 15)
            }
        );
        assert_eq!(
            Sensor::parse(
                "Sensor at x=3011731, y=1976307: closest beacon is at x=2729595, y=2000000"
            ),
            Sensor {
                position: ivec2(3011731, 1976307),
                closest_beacon: ivec2(2729595, 2000000)
            }
        );
    }

    #[test]
    fn test_perimiter_sensor() {
        let sensor = Sensor {
            position: IVec2::ZERO,
            closest_beacon: ivec2(3, 0),
        };
        let per = sensor.perimeter();
        for y in -4..=4 {
            for x in -4..=4 {
                let p = ivec2(x, y);
                if per.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    #[test]
    fn test_parse_map() {
        Map::parse(EXAMPLE);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE, 10), 26);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE, 20), 56000011);
    }

    const EXAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
}
