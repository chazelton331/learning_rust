use std::thread;

fn main() {
    for i in 0..10 {
        let answer = i * i;

        thread::sleep_ms(500);

        println!("{}", answer);
    }
}
