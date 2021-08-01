use shared::{
    intcode::{Computer, Program},
    read_string,
};

fn main() {
    let contents = read_string("day_02/data/input.txt").unwrap();
    let program = contents.parse::<Program>().unwrap();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = Computer::new(program.clone());
            computer.set(1, noun).unwrap();
            computer.set(2, verb).unwrap();
            computer.run().unwrap();
            if computer.get(0).unwrap() == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
