fn main() {
    println!("01: {}", part01(include_str!("../inputs/02")));
    println!("02: {}", part02(include_str!("../inputs/02")));
}

#[derive(Debug, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

fn part01(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let (x, y) = line.split_once(' ').unwrap();

        let x = if x == "A" {
            Rps::Rock
        } else if x == "B" {
            Rps::Paper
        } else if x == "C" {
            Rps::Scissors
        } else {
            panic!("Couldn't map {x} to a Rps")
        };

        let y = if y == "X" {
            Rps::Rock
        } else if y == "Y" {
            Rps::Paper
        } else if y == "Z" {
            Rps::Scissors
        } else {
            panic!("Couldn't map {y} to a Rps")
        };

        score += match y {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };

        score += match (x, y) {
            (Rps::Rock, Rps::Rock) => 3,
            (Rps::Rock, Rps::Paper) => 6,
            (Rps::Rock, Rps::Scissors) => 0,
            (Rps::Paper, Rps::Rock) => 0,
            (Rps::Paper, Rps::Paper) => 3,
            (Rps::Paper, Rps::Scissors) => 6,
            (Rps::Scissors, Rps::Rock) => 6,
            (Rps::Scissors, Rps::Paper) => 0,
            (Rps::Scissors, Rps::Scissors) => 3,
        }
    }
    return score;
}

fn part02(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let (x, y) = line.split_once(' ').unwrap();

        let x = match x {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("Couldn't map {x} to a Rps"),
        };

        let y = match y {
            // lose
            "X" => match x {
                Rps::Rock => Rps::Scissors,
                Rps::Paper => Rps::Rock,
                Rps::Scissors => Rps::Paper,
            },
            // draw
            "Y" => x.clone(),
            // win
            "Z" => match x {
                Rps::Rock => Rps::Paper,
                Rps::Paper => Rps::Scissors,
                Rps::Scissors => Rps::Rock,
            },
            _ => panic!("Couldn't map {y} to a Rps"),
        };

        score += match y {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };

        score += match (x, y) {
            (Rps::Rock, Rps::Rock) => 3,
            (Rps::Rock, Rps::Paper) => 6,
            (Rps::Rock, Rps::Scissors) => 0,
            (Rps::Paper, Rps::Rock) => 0,
            (Rps::Paper, Rps::Paper) => 3,
            (Rps::Paper, Rps::Scissors) => 6,
            (Rps::Scissors, Rps::Rock) => 6,
            (Rps::Scissors, Rps::Paper) => 0,
            (Rps::Scissors, Rps::Scissors) => 3,
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(part01(input), 15);
    }

    #[test]
    fn example02() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(part02(input), 12);
    }
}
