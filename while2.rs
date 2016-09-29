fn main() {
    let mut x = 10;

    loop {
        x += x * 2;
        println!("x={}", x);

        if x > 1000 { break; }
    }
}
