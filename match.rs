use std::cmp::Ordering;
use std::env;

fn print_usage(prog_name: &str) {
    println!("\nusage:\n\n\t {} <blahblah>", prog_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(args[0]);
        env::set_exit_status(1);
    } else {
        let x = 123;
        let y = 567;

        match x.cmp(&y) {
            Ordering::Less    => println!("{} was smaller than {}",  x, y),
            Ordering::Greater => println!("{} was bigger than {}",   x, y),
            Ordering::Equal   => println!("{} was the same than {}", x, y)
        }
    }
}
