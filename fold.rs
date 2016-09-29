fn main() {
    let answer = (1..10).fold(1, |accumulator, element| accumulator * element);
    let sum    = (1..10).fold(0, |accumulator, element| accumulator + element);

    println!("{} ", answer);
    println!("{} ", sum);
}
