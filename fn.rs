fn add_em_up(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let x = 1;
    let y = 2;

    let (a, b) = (10, 20);

    println!("greetings, x={} and y={}", x, y);

    let answer: i32 = add_em_up(a, x);

    println!("addem: {}", answer);
}
