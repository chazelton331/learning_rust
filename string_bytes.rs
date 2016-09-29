use std::str;

fn main() {
    let x: &[u8] = &[b'c', b'l', b'i', b'f', b'f'];

    let stack_str: &str = str::from_utf8(x).unwrap();

    println!("{}", stack_str);
}
