fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(i: u32) -> String {
    if i % 5 == 0 && i % 3 == 0 {
        "FizzBuzz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else {
        i.to_string()
    }
}
