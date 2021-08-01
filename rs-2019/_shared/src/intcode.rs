use std::{num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Program(Vec<i64>);

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl From<Vec<i64>> for Program {
    fn from(value: Vec<i64>) -> Self {
        Self(value)
    }
}

impl Into<Vec<i64>> for Program {
    fn into(self) -> Vec<i64> {
        self.0
    }
}

pub struct Computer {
    memory: Vec<i64>,
    cursor: usize,
}

impl Computer {
    /// Creates a `Computer` with the provided program in its memory.
    pub fn new<P>(program: P) -> Self
    where
        P: Into<Program>,
    {
        Self {
            memory: program.into().clone().into(),
            cursor: 0,
        }
    }

    /// Returns the current cursor position.
    /// # Example
    /// ```
    /// use shared::intcode::Computer;
    /// let mut computer = Computer::new(vec![]);
    /// assert_eq!(0, computer.tell());
    /// ```
    pub fn tell(&self) -> usize {
        self.cursor
    }

    /// Returns the value in memory at the current curspr position. Increments cursor position.
    /// # Example
    /// ```
    /// use shared::intcode::Computer;
    /// let mut computer = Computer::new(vec![42]);
    /// assert_eq!(42, computer.read().unwrap());
    /// assert_eq!(1, computer.tell());
    /// ```
    pub fn read(&mut self) -> Result<i64> {
        let result = self
            .memory
            .get(self.cursor)
            .ok_or(anyhow!("Cursor out of bounds!"))?;
        self.cursor += 1;
        Ok(*result)
    }

    /// Returns the value in memory at the provided position. Does not move cursor.
    /// # Example
    /// ```
    /// use shared::intcode::Computer;
    /// let mut computer = Computer::new(vec![42]);
    /// assert_eq!(0, computer.tell());
    /// assert_eq!(42, computer.get(0).unwrap());
    /// assert_eq!(0, computer.tell());
    /// ```
    pub fn get(&self, pos: usize) -> Result<i64> {
        self.memory
            .get(pos)
            .ok_or(anyhow!("Position out of bounds!"))
            .map(|x| *x)
    }

    /// Sets the value in memeory at the provided position the provided value. Does not move cursor.
    /// # Example
    /// ```
    /// use shared::intcode::Computer;
    /// let mut computer = Computer::new(vec![42]);
    /// computer.set(0, 69).unwrap();
    /// assert_eq!(69, computer.read().unwrap());
    /// ```
    pub fn set(&mut self, pos: usize, val: i64) -> Result<()> {
        let cur = self
            .memory
            .get_mut(pos)
            .ok_or(anyhow!("Position out of bounds!"))?;
        *cur = val;
        Ok(())
    }

    /// Tries to execute the operation at the current cursor position.
    /// # Example
    /// ```
    /// use shared::intcode::Computer;
    /// let mut computer = Computer::new(vec![1,0,0,3]);
    /// computer.step().unwrap();
    /// assert_eq!(2, computer.get(3).unwrap());
    /// assert_eq!(4, computer.tell());
    /// ```
    pub fn step(&mut self) -> Result<i64> {
        match self.read()? {
            1 => {
                let a1 = self.read()?;
                let a2 = self.read()?;
                let a3 = self.read()?;
                let v1 = self.get(a1 as usize)?;
                let v2 = self.get(a2 as usize)?;
                self.set(a3 as usize, v1 + v2)?;
            }
            2 => {
                let a1 = self.read()?;
                let a2 = self.read()?;
                let a3 = self.read()?;
                let v1 = self.get(a1 as usize)?;
                let v2 = self.get(a2 as usize)?;
                self.set(a3 as usize, v1 * v2)?;
            }
            99 => {
                return Ok(1);
            }
            _ => return Err(anyhow!("Unknown opcode!")),
        }

        Ok(0)
    }

    /// Steps until the halt opcode is reached or an error occurs.
    /// # Example
    /// ```
    /// use shared::intcode::Computer;
    /// let mut computer = Computer::new(vec![1,0,0,3,1,4,4,7,99]);
    /// computer.run().unwrap();
    /// assert_eq!(2, computer.get(3).unwrap());
    /// assert_eq!(2, computer.get(7).unwrap());
    /// ```
    pub fn run(&mut self) -> Result<()> {
        while self.step()? == 0 {}
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program_from_string() {
        assert_eq!(vec![0, 1, 2], "0,1,2".parse::<Program>().unwrap().0);
        assert!("s,1,2".parse::<Program>().is_err());
    }

    #[test]
    fn test_run_programs() {
        let mut computer = Computer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(0, computer.step().unwrap());
        assert_eq!(70, computer.get(3).unwrap());
        assert_eq!(0, computer.step().unwrap());
        assert_eq!(3500, computer.get(0).unwrap());
        assert_eq!(1, computer.step().unwrap());

        let mut computer = Computer::new(vec![1, 0, 0, 0, 99]);
        computer.run().unwrap();
        assert_eq!(vec![2, 0, 0, 0, 99], computer.memory);

        let mut computer = Computer::new(vec![2, 3, 0, 3, 99]);
        computer.run().unwrap();
        assert_eq!(vec![2, 3, 0, 6, 99], computer.memory);

        let mut computer = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        computer.run().unwrap();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.memory);

        let mut computer = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run().unwrap();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.memory);
    }
}
