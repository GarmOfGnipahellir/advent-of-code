use shared::{
    intcode::{Computer, Program},
    read_string,
};

fn main() {
    let contents = read_string("day_02/data/input.txt").unwrap();
    let program = contents.parse::<Program>().unwrap();
    let mut computer = Computer::new(program);
    computer.set(1, 12).unwrap();
    computer.set(2, 2).unwrap();
    computer.run().unwrap();
    println!("{}", computer.get(0).unwrap());
}
