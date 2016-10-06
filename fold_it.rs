fn main() {
    let accumulated = (1..100).fold(0, |acc, n| acc + (n * 100));

    println!("{}", accumulated);
}
