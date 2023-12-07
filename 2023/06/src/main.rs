#![feature(test)]

const INP: &str = include_str!("../in.txt");

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    fn parse(inp: &str) -> Self {
        let (time, distance) = inp.trim().split_once("\n").unwrap();
        let time = time
            .trim()
            .replace("Time:", "")
            .replace(" ", "")
            .parse::<usize>()
            .unwrap();
        let distance = distance
            .trim()
            .replace("Distance:", "")
            .replace(" ", "")
            .parse::<usize>()
            .unwrap();
        Self::new(time, distance)
    }

    fn calc_record_range(&self) -> (usize, usize) {
        let t = self.time as f32;
        let d = self.distance as f32;
        let low = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
        let high = (t + (t * t - 4.0 * d).sqrt()) / 2.0;
        (low.floor() as usize, high.ceil() as usize)
    }
}

fn parse_races(inp: &str) -> Vec<Race> {
    let (times, distances) = inp.trim().split_once("\n").unwrap();
    times
        .replace("Time:", "")
        .split_whitespace()
        .zip(distances.replace("Distance:", "").split_whitespace())
        .map(|(time, distance)| {
            (
                time.parse::<usize>().unwrap(),
                distance.parse::<usize>().unwrap(),
            )
        })
        .map(|(time, distance)| Race::new(time, distance))
        .collect::<Vec<_>>()
}

fn part1(inp: &str) -> usize {
    parse_races(inp)
        .iter()
        .map(|r| {
            let (low, high) = r.calc_record_range();
            high - (low + 1)
        })
        .product()
}

fn part2(inp: &str) -> usize {
    let race = Race::parse(inp);
    let (low, high) = race.calc_record_range();
    high - (low + 1)
}

fn main() {
    println!("Part 1: {}", part1(INP));
    println!("Part 2: {}", part2(INP));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;

    const EX1: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    impl Race {
        fn calc_distance(&self, speed: usize) -> usize {
            (self.time - speed) * speed
        }
    }

    #[test]
    fn test_race_parse() {
        assert_eq!(Race::parse(EX1), Race::new(71530, 940200))
    }

    #[test]
    fn test_race_calc_distance() {
        let race = Race::new(7, 9);
        assert_eq!(race.calc_distance(0), 0);
        assert_eq!(race.calc_distance(1), 6);
        assert_eq!(race.calc_distance(2), 10);
        assert_eq!(race.calc_distance(3), 12);
        assert_eq!(race.calc_distance(4), 12);
        assert_eq!(race.calc_distance(5), 10);
        assert_eq!(race.calc_distance(6), 6);
        assert_eq!(race.calc_distance(7), 0);
    }

    #[test]
    fn test_race_calc_record_range() {
        assert_eq!(Race::new(7, 9).calc_record_range(), (1, 6));
    }

    #[test]
    fn test_parse_races() {
        assert_eq!(
            parse_races(EX1),
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX1), 71503);
    }

    #[bench]
    fn bench_part2_brute_force(b: &mut Bencher) {
        b.iter(|| {
            let race = Race::parse(INP);
            (0..=race.time)
                .filter(|&s| race.calc_distance(s) > race.distance)
                .count()
        })
    }

    #[bench]
    fn bench_part2_optimized(b: &mut Bencher) {
        b.iter(|| part2(INP))
    }
}
