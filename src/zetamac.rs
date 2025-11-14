use std::fmt;

use rand::prelude::*;

#[derive(Debug, Default)]
pub struct GameState {
    pub current_problem: Problem,
    pub seed: i32,
    pub score: u32,
    pub time: u32,
    settings: GameSettings,
    rng: ThreadRng,
}

#[derive(Debug)]
struct GameSettings {
    addition_range: (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>),
    multiplication_range: (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>),
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            addition_range: (2..=10, 2..=100),
            multiplication_range: (2..=12, 2..=100),
        }
   }
}

#[derive(Debug, Default)]
pub struct Problem {
    first: u32,
    second: u32,
    answer: u32,
    operation: Operation,
}

impl fmt::Display for Problem {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} = {}", self.first, self.operation.char(), self.second, self.answer)
   }
}

impl GameState {
    pub fn create_next_problem(&mut self) {
        let options = [Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide];
        let operation = options.choose(&mut self.rng).copied().unwrap_or(Operation::Add);

        self.current_problem = match operation {
            Operation::Add | Operation::Subtract => {
                let numbers = (
                    self.rng.random_range(self.settings.addition_range.0.clone()),
                    self.rng.random_range(self.settings.addition_range.1.clone())
                );
                let sum = numbers.0 + numbers.1;

                if operation == Operation::Add {
                    Problem { first: numbers.0, second: numbers.1, answer: sum, operation }
                } else {
                    Problem { first: sum, second: numbers.0, answer: numbers.1, operation }
                }
            },
            Operation::Multiply | Operation::Divide => {
                let numbers = (
                    self.rng.random_range(self.settings.multiplication_range.0.clone()),
                    self.rng.random_range(self.settings.multiplication_range.1.clone())
                );
                let product = numbers.0 * numbers.1;

                if operation == Operation::Multiply {
                    Problem { first: numbers.0, second: numbers.1, answer: product, operation }
                } else {
                    Problem { first: product, second: numbers.0, answer: numbers.1, operation }
                }
            }
        };
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Operation {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn char(&self) -> char {
        match *self {
            Operation::Add => '+',
            Operation::Subtract => '-',
            Operation::Multiply => '*',
            Operation::Divide => '/',
        }
    }
}


