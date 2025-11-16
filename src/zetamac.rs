use chrono::Utc;
use rand::{prelude::*, random};
use rand_chacha::ChaCha8Rng;
use std::cmp::max;
use std::fmt;

#[derive(Debug)]
pub struct GameState {
    pub current_problem: Problem,
    pub seed: u64,
    pub score: u32,
    pub start_time: Option<i64>,
    pub settings: GameSettings,
    rng: ChaCha8Rng,
}

#[derive(Debug, Default)]
pub struct Problem {
    first: u32,
    second: u32,
    answer: u32,
    operation: Operation,
}

#[derive(Debug)]
struct GameSettings {
    total_time_seconds: i64,
    addition_range: (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>),
    multiplication_range: (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>),
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Operation {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            addition_range: (2..=10, 2..=100),
            multiplication_range: (2..=12, 2..=100),
            total_time_seconds: 120,
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.first,
            self.operation.char(),
            self.second
        )
    }
}

impl GameState {
    pub fn new() -> GameState {
        let seed: u64 = random();
        let settings = GameSettings {
            ..Default::default()
        };

        GameState::from_options(seed, settings)
    }

    pub fn start_game(&mut self) {
        self.start_time = Some(Utc::now().timestamp());
    }

    pub fn clock_time(&self) -> i64 {
        if let Some(start_time) = self.start_time {
            max(
                self.settings.total_time_seconds - (Utc::now().timestamp() - start_time),
                0,
            )
        } else {
            self.settings.total_time_seconds
        }
    }

    fn from_options(seed: u64, settings: GameSettings) -> GameState {
        let mut game_state = GameState {
            score: 0,
            start_time: None,
            current_problem: Problem {
                ..Default::default()
            },
            rng: ChaCha8Rng::seed_from_u64(seed),
            settings,
            seed,
        };
        game_state.next_problem();
        game_state
    }

    pub fn is_correct(&self, user_input: &String) -> bool {
        self.current_problem.answer.to_string() == *user_input
    }

    pub fn next_problem(&mut self) {
        let options = [
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide,
        ];
        let operation = options
            .choose(&mut self.rng)
            .copied()
            .unwrap_or(Operation::Add);

        self.current_problem = match operation {
            Operation::Add | Operation::Subtract => {
                let numbers = (
                    self.rng
                        .random_range(self.settings.addition_range.0.clone()),
                    self.rng
                        .random_range(self.settings.addition_range.1.clone()),
                );
                let sum = numbers.0 + numbers.1;

                if operation == Operation::Add {
                    Problem {
                        first: numbers.0,
                        second: numbers.1,
                        answer: sum,
                        operation,
                    }
                } else {
                    Problem {
                        first: sum,
                        second: numbers.0,
                        answer: numbers.1,
                        operation,
                    }
                }
            }
            Operation::Multiply | Operation::Divide => {
                let numbers = (
                    self.rng
                        .random_range(self.settings.multiplication_range.0.clone()),
                    self.rng
                        .random_range(self.settings.multiplication_range.1.clone()),
                );
                let product = numbers.0 * numbers.1;

                if operation == Operation::Multiply {
                    Problem {
                        first: numbers.0,
                        second: numbers.1,
                        answer: product,
                        operation,
                    }
                } else {
                    Problem {
                        first: product,
                        second: numbers.0,
                        answer: numbers.1,
                        operation,
                    }
                }
            }
        };
    }
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
