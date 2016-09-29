#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}


fn main() {
    //let options: SomeOptions = Default::default();
    let options = SomeOptions { foo: 42, ..Default::default() };

    println!("{}, {}", options.foo, options.bar);
}
