fn main() {
    let mut x: i32 = 123;

    println!("{}", x);

    for y in 1..10 {
        x += 1;
        println!("{} {}", y, x);
    }
}
