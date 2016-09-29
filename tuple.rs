fn check_if_30(a: i32, b: i32) {
    if a + b == 30 {
        println!("oh boy!");
    }
}

fn main() {
    let (a, b) = (10, 20);

    check_if_30(a, b);
}
