fn main() {
    let mut i = 1;

    while i <= 100 {
        println!("{}", fizzbuzz(i));

        i += 1;
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
