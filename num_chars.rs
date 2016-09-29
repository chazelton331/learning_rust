#![feature(env)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for i in args {
            println!("{}", args[i]);
        }
        //println!("The first argument is {}", args[1]);
    } else {
        println!("Err!  Need at least one argument to count the length");
    }
}
