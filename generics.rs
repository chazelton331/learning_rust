fn what_is_it<T>(x: T) {
    match x {
        123     => println!("i32"),
        123.0   => println!("f64"),
    }
}

fn main() {
    let x: Option<i32> = Some(5);

    match x {
        Some(x1)    => println!("{}", x1),
        None        => println!("nothing"),
    }

    what_is_it(5);
    what_is_it(5.55);
}
