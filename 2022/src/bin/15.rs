use std::{collections::HashSet, fmt::Display};

use glam::{ivec2, IVec2};
use rayon::prelude::*;
use regex::Regex;

fn main() {
    println!("01: {}", part01(include_str!("../inputs/15"), 2000000));
    println!("02: {}", part02(include_str!("../inputs/15")));
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
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<IVec2>,
    empty: HashSet<IVec2>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let sensors = s
            .lines()
            .map(|line| Sensor::parse(line))
            .collect::<Vec<_>>();
        let beacons = sensors.iter().map(|sensor| sensor.closest_beacon).collect();
        Self {
            sensors,
            beacons,
            empty: HashSet::new(),
        }
    }

    fn min_max(&self) -> (IVec2, IVec2) {
        let (mut min, mut max) = (IVec2::ONE * i32::MAX, IVec2::ONE * i32::MIN);
        let positions = self
            .sensors
            .iter()
            .map(|s| s.position)
            .chain(self.beacons.iter().map(|p| *p))
            .chain(self.empty.iter().map(|p| *p))
            .collect::<Vec<_>>();
        for p in positions {
            min.x = i32::min(p.x, min.x);
            min.y = i32::min(p.y, min.y);
            max.x = i32::max(p.x, max.x);
            max.y = i32::max(p.y, max.y);
        }
        (min, max)
    }

    fn calc_empty(&mut self) {
        // self.empty.clear();

        // let sensor_iter = self.sensors.par_iter().flat_map(|sensor| {
        //     let closest_delta = sensor.closest_beacon - sensor.position;
        //     let closest_dist = closest_delta.x.abs() + closest_delta.y.abs();

        //     println!("Calcing sensor...");

        //     (-closest_dist..=closest_dist)
        //         .into_par_iter()
        //         .flat_map(|y| {
        //             (-closest_dist..=closest_dist)
        //                 .into_par_iter()
        //                 .map(move |x| ivec2(x, y))
        //                 .collect::<Vec<_>>()
        //         })
        //         .filter_map(|delta| {
        //             let p = sensor.position + delta;
        //             let dist = delta.x.abs() + delta.y.abs();
        //             if dist <= closest_dist
        //                 && self
        //                     .sensors
        //                     .par_iter()
        //                     .find_any(|&s| s.position == p)
        //                     .is_none()
        //                 && !self.beacons.contains(&p)
        //             {
        //                 Some(p)
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect::<Vec<_>>()
        // });

        // self.empty.par_extend(sensor_iter);

        for sensor in &self.sensors {
            let closest_delta = sensor.closest_beacon - sensor.position;
            let closest_dist = closest_delta.x.abs() + closest_delta.y.abs();

            println!("Calcing {sensor:?} {closest_dist}...");

            for y in -closest_dist..=closest_dist {
                for x in -closest_dist..=closest_dist {
                    let delta = ivec2(x, y);
                    let p = sensor.position + delta;
                    let dist = delta.x.abs() + delta.y.abs();
                    if dist <= closest_dist
                        && self.sensors.iter().find(|&s| s.position == p).is_none()
                        && !self.beacons.contains(&p)
                    {
                        self.empty.insert(p);
                    }
                }
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.min_max();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let p = ivec2(x, y);
                if self.sensors.iter().find(|&s| s.position == p).is_some() {
                    write!(f, "S")?;
                } else if self.beacons.contains(&p) {
                    write!(f, "B")?;
                } else if self.empty.contains(&p) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part01(input: &str, row: i32) -> i32 {
    let mut map = Map::parse(input);
    let (min, max) = map.min_max();
    println!("{min:?}, {max:?}");
    map.calc_empty();
    let (min, max) = map.min_max();
    println!("{min:?}, {max:?}");
    let mut res = 0;
    for x in min.x..=max.x {
        if map.empty.contains(&ivec2(x, row)) {
            res += 1;
        }
    }
    res
}

fn part02(input: &str) -> i32 {
    unimplemented!()
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
    fn test_parse_map() {
        println!("{}", Map::parse(EXAMPLE));
    }

    #[test]
    fn test_calc_empty_map() {
        let mut map = Map::parse(EXAMPLE);
        map.calc_empty();
        let fmt = format!("{map}");
        println!("{fmt}");
        assert_eq!(
            fmt.trim(),
            r#"
..........#..........................
.........###.........................
........#####........................
.......#######.......................
......#########.............#........
.....###########...........###.......
....#############.........#####......
...###############.......#######.....
..#################.....#########....
.###################.#.###########...
##########S########################..
.###########################S#######.
..###################S#############..
...###################SB##########...
....#############################....
.....###########################.....
......#########################......
.......#########S#######S#####.......
........#######################......
.......#########################.....
......####B######################....
.....###S#############.###########...
......#############################..
.......#############################.
.......#############S#######S########
......B#############################.
.....############SB################..
....##################S##########B...
...#######S######################....
....############################.....
.....#############S######S######.....
......#########################......
.......#######..#############B.......
........#####....###..#######........
.........###......#....#####.........
..........#.............###..........
.........................#...........
"#
            .trim()
        );
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE, 10), 26);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
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
