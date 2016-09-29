fn print_string(slice: &str) {
    println!("{}", slice);
}

fn main() {
    let mut s = "Hello".to_string();
    s.push_str(", world!");
    print_string(&*s);
}
