mod game;

use game::fizzbuzz::FizzBuzz;

fn main() {
    for i in 1..=100 {
        println!("{}", FizzBuzz::from(i));
    }
}
