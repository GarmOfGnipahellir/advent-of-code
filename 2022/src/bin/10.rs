fn main() {
    println!("01: {}", part01(include_str!("../inputs/10")));
    println!("02:");
    part02(include_str!("../inputs/10"));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        match s {
            "noop" => Self::NoOp,
            _ if s.starts_with("addx") => s
                .split_once(' ')
                .and_then(|(_, x)| x.parse::<i32>().ok())
                .map(|x| Self::AddX(x))
                .unwrap(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Program(Vec<Instruction>);

impl Program {
    fn parse(s: &str) -> Self {
        let mut instructions = Vec::new();
        for line in s.lines() {
            instructions.push(Instruction::parse(line));
        }
        Self(instructions)
    }
}

struct Cpu {
    program: Program,
    instruction_index: usize,
    instruction_start: usize,
    cycle: usize,
    x: i32,
}

impl Cpu {
    fn new(program: Program) -> Self {
        Self {
            program,
            instruction_index: 0,
            instruction_start: 0,
            cycle: 0,
            x: 1,
        }
    }

    fn tick(&mut self) {
        let instruction = self.program.0[self.instruction_index];
        let cycles_since_instruction_start = self.cycle - self.instruction_start;
        match instruction {
            Instruction::NoOp => {
                if cycles_since_instruction_start >= 1 {
                    self.instruction_index += 1;
                    self.instruction_start = self.cycle;
                    // println!("noop finished execution at the end of {}", self.cycle);
                }
            }
            Instruction::AddX(x) => {
                if cycles_since_instruction_start >= 2 {
                    self.x += x;
                    self.instruction_index += 1;
                    self.instruction_start = self.cycle;
                    // println!("addx {} finished execution at the end of {}", x, self.cycle);
                }
            }
        }
        self.cycle += 1;
    }
}

fn part01(input: &str) -> i32 {
    let mut cpu = Cpu::new(Program::parse(input));
    let mut signal_strengths = Vec::new();
    while cpu.instruction_index < cpu.program.0.len() {
        cpu.tick();

        if [20, 60, 100, 140, 180, 220].contains(&cpu.cycle) {
            signal_strengths.push(cpu.cycle as i32 * cpu.x);
        }
    }
    signal_strengths.iter().sum()
}

fn part02(input: &str) {
    let mut cpu = Cpu::new(Program::parse(input));
    let mut row = Vec::with_capacity(40);
    for _ in 0..240 {
        cpu.tick();

        let i = row.len() as i32;

        if i >= cpu.x - 1 && i <= cpu.x + 1 {
            row.push("#")
        } else {
            row.push(" ");
        }

        if row.len() == 40 {
            println!("{}", row.join(""));
            row.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Instruction::parse("noop"), Instruction::NoOp);
        assert_eq!(Instruction::parse("addx 3"), Instruction::AddX(3));
        assert_eq!(Instruction::parse("addx -5"), Instruction::AddX(-5));

        let input = r#"noop
addx 3
addx -5"#;
        assert_eq!(
            Program::parse(input),
            Program(vec![
                Instruction::NoOp,
                Instruction::AddX(3),
                Instruction::AddX(-5)
            ])
        );
    }

    #[test]
    fn test_tick() {
        let mut cpu = Cpu::new(Program(vec![
            Instruction::NoOp,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ]));
        assert_eq!(cpu.cycle, 0);
        assert_eq!(cpu.x, 1);
        cpu.tick();
        assert_eq!(cpu.cycle, 1);
        assert_eq!(cpu.x, 1);
        cpu.tick();
        assert_eq!(cpu.cycle, 2);
        assert_eq!(cpu.x, 1);
        cpu.tick();
        assert_eq!(cpu.cycle, 3);
        assert_eq!(cpu.x, 1);
        cpu.tick();
        assert_eq!(cpu.cycle, 4);
        assert_eq!(cpu.x, 4);
        cpu.tick();
        assert_eq!(cpu.cycle, 5);
        assert_eq!(cpu.x, 4);
        cpu.tick();
        assert_eq!(cpu.cycle, 6);
        assert_eq!(cpu.x, -1);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), 13140);
    }

    #[test]
    fn example02() {
        part02(EXAMPLE)
    }

    const EXAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
}
