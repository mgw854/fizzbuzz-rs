use std::fmt::Display;

pub enum FizzBuzz {
    FizzBuzz,
    Fizz,
    Buzz,
    Number { x: u32 },
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FizzBuzz::FizzBuzz => f.write_str("FizzBuzz"),
            FizzBuzz::Fizz => f.write_str("Fizz"),
            FizzBuzz::Buzz => f.write_str("Buzz"),
            FizzBuzz::Number { x } => f.write_str(&x.to_string()),
        }
    }
}

impl From<u32> for FizzBuzz {
    fn from(i: u32) -> Self {
        match (i % 3, i % 5) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (0, _) => FizzBuzz::Fizz,
            (_, 0) => FizzBuzz::Buzz,
            _ => FizzBuzz::Number { x: i },
        }
    }
}
