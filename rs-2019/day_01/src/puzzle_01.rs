use shared::read_string;

fn main() {
    let contents = read_string("day_01/data/input.txt").unwrap();

    let answer: i64 = contents
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .map(|x| (x / 3) - 2)
        .sum();

    println!("{}", answer);
}