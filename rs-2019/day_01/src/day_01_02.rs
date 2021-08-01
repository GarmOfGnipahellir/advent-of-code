use shared::read_string;

fn calc_fuel(mass: i64) -> i64 {
    let mut mass = (mass / 3) - 2;
    let mut fuel = 0;

    while mass > 0 {
        fuel += mass;
        mass = (mass / 3) - 2;
    }

    fuel
}

fn main() {
    let contents = read_string("day_01/data/01_input.txt").unwrap();

    let answer: i64 = contents
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .map(calc_fuel)
        .sum();

    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(2, calc_fuel(14));
        assert_eq!(966, calc_fuel(1969));
        assert_eq!(50346, calc_fuel(100756));
    }
}
