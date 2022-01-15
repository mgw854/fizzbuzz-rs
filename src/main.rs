mod game;

use game::fizzbuzz::FizzBuzz;

fn main() {
    for y in (1..=100).into_iter().map(|x| FizzBuzz::from(x)) {
        println!("{}", y);
    }
}
