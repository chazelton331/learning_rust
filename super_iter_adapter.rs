fn main() {
    let nums = (1..)
                .filter(|&n| n % 2 == 0)
                .filter(|&n| n > 100)
                .take(5)
                .collect::<Vec<_>>();

    for num in nums {
        println!("{}", num);
    }
}
