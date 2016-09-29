fn main() {
    let mut terms = vec![ "sunrise", "hiking" ];

    terms.push("abc");

    for t in &terms {
        println!("{}", t);
    }
}
