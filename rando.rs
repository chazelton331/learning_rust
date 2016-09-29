use std::rand;

fn main() {
    println!("{}", (rand::random::<u32>() % 100) + 1);
}
