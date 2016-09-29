fn main() {
    let collection_of_numbers = (1..101).collect::<Vec<i32>>();

    for num in &collection_of_numbers {
        print!("{} ", num);
    }

    let big_numbers = (1..101).find(|n| *n > 42);

    match big_numbers {
        Some(n) => { print!("big {} ", n)   },
        None    => { print!("-")            },
    }

    println!("");

                   /* fold(base, |accumulator, element| ...) */
    let sum = (1..10).fold(0, |sum, n| sum + n);

    println!("sum={} ", sum);
}
