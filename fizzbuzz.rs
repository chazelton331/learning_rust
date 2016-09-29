use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
enum FizzBuzzValue {
    Value(u64),
    Fizz(u64),
    Buzz(u64),
    FizzBuzz(u64),
}

impl Display for FizzBuzzValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "hi")
    }
}

fn to_fizzbuzz(n: u64) -> FizzBuzzValue {
    match n {
        n if n % 15 == 0 => FizzBuzzValue::FizzBuzz(n),
        n if n %  5 == 0 => FizzBuzzValue::Fizz(n),
        n if n %  3 == 0 => FizzBuzzValue::Buzz(n),
        n => n
    }
}

fn main() {
    for n in (1..101).map(to_fizzbuzz) {
        println!("{}", n)
    }
}
