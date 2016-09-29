#![feature(io)]   /* silences "warning: use of unstable library feature 'io'" */

use std::old_io;

fn main() {
    print!("Tell me something: ");

    let input = old_io::stdin().read_line()
                               .ok()
                               .expect("Ugh, couldn't read what you wrote.");

    println!("You said: {}", input);
}
