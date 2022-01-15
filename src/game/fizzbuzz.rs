use std::fmt::Display;

use num::{FromPrimitive, Integer, Unsigned};

#[derive(PartialEq, Eq, Debug)]
pub enum FizzBuzz<T>
where
    T: Unsigned + Integer,
{
    FizzBuzz,
    Fizz,
    Buzz,
    Number { x: T },
}

impl<T> Display for FizzBuzz<T>
where
    T: Unsigned + Integer + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FizzBuzz::FizzBuzz => f.write_str("FizzBuzz"),
            FizzBuzz::Fizz => f.write_str("Fizz"),
            FizzBuzz::Buzz => f.write_str("Buzz"),
            FizzBuzz::Number { x } => f.write_str(&x.to_string()),
        }
    }
}

impl<T> From<T> for FizzBuzz<T>
where
    T: Unsigned + Integer + FromPrimitive + Copy,
{
    fn from(i: T) -> Self {
        if i % T::from_u32(15).unwrap() == T::zero() {
            FizzBuzz::FizzBuzz
        } else if i % T::from_u32(5).unwrap() == T::zero() {
            FizzBuzz::Buzz
        } else if i % T::from_u32(3).unwrap() == T::zero() {
            FizzBuzz::Fizz
        } else {
            FizzBuzz::Number { x: i }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_1_number() {
        assert_eq!(FizzBuzz::Number { x: 1 as u32 }, FizzBuzz::from(1))
    }

    #[test]
    fn from_3_number() {
        assert_eq!(FizzBuzz::Fizz, FizzBuzz::from(3 as u32))
    }

    #[test]
    fn from_5_number() {
        assert_eq!(FizzBuzz::Buzz, FizzBuzz::from(5 as u32))
    }

    #[test]
    fn from_15_number() {
        assert_eq!(FizzBuzz::FizzBuzz, FizzBuzz::from(15 as u32))
    }

    #[test]
    fn display_1_number() {
        assert_eq!("1", FizzBuzz::Number { x: 1 as u32 }.to_string())
    }

    #[test]
    fn display_3_number() {
        assert_eq!("Fizz", FizzBuzz::<u32>::Fizz.to_string())
    }

    #[test]
    fn display_5_number() {
        assert_eq!("Buzz", FizzBuzz::<u32>::Buzz.to_string())
    }

    #[test]
    fn display_15_number() {
        assert_eq!("FizzBuzz", FizzBuzz::<u32>::FizzBuzz.to_string())
    }
}
