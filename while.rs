fn main() {
    let mut x = 10;
    let mut done = false;

    while !done {
        x += x * 2;
        println!("x={}", x);

        if x > 1000 {
            done = true;
        }
    }
}
