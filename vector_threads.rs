//use std::sync::{Arc,Mutex};
//use std::vec::Vec;


fn print_thread_name(name: &str) {
    let new_name = name.to_string() + " ok";

    println!("{}", new_name);
}

fn main() {
    let number_of_threads   = 20;
    let mut thread_names    = Vec::new();

    for n in 0..number_of_threads {
        let thread_name = format!("thread-{}", n + 1);
        thread_names.push(thread_name);
    }

    for n in &thread_names {
        print_thread_name(n);
    }
}
