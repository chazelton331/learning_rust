fn main() {
    let nums = vec![ 1, 2, 3 ];

    for num in &nums {
        println!("{}",  num);
        println!("{}", *num);
    }

    for num in nums {
        println!("{}", num);
    }
}
