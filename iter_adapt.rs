fn main() {
    let new_iterator = (1..21).map(|n| n + 20);

    for num in new_iterator {
        println!("{}", num);
    }

    println!("--");

    for num in (1..).take(5) {
        println!("{}", num);
    }

    println!("--");

    for num in (1..22).filter(|n| n % 2 == 0) {
        println!("{}", num);
    }
}
