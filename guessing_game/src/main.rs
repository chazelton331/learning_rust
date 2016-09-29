#![feature(io)]
#![feature(rand)]

use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn cmp(n1: u32, n2: u32) -> Ordering {
    if      n1 < n2 { Ordering::Less    }
    else if n1 > n2 { Ordering::Greater }
    else            { Ordering::Equal   }
}


fn main() {
    println!("---------------------------------------------");
    print!("I'm thinking of a number.  Guess what it is: ");

    let secret_number = (rand::random::<u32>() % 100) + 1;

    loop {
        let input = old_io::stdin().read_line()
                                   .ok()
                                   .expect("No input detected.");

        println!("Your guess is: {}", input);

        let parsed_number: Result<u32, _> = input.trim().parse();

        let number = match parsed_number {
            Ok(number) => number,
            Err(_)     => {
                println!("You didn't give me a number, dummy.");
                return;
            }
        };

        match cmp(number, secret_number) {
            Ordering::Less    => {
                println!("Your guess was too small.");
                print!("Guess again: ");
            },
            Ordering::Greater => {
                println!("Your guess was too big.");
                print!("Guess again: ");
            },
            Ordering::Equal   => {
                println!("You got it!");
                break;
            }
        };
    }
}
