use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Inside a thread"
    });

    println!("{}", handle.join().unwrap());
}
