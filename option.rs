fn some_or_none(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        None    => -1,
    }
}

fn some_or_none_type(opt: Option<String>) -> String {
    let ret = match opt {
        Some(x) => x,
        None    => "nothing".to_string(),
    };

    ret
}

fn main() {
    println!("{}", some_or_none(None));
    println!("{}", some_or_none(Some(5)));

    println!("{}", some_or_none_type(None));
    println!("{}", some_or_none_type(Some("hi there".to_string())));

    let str = "Foo bar".to_string();

    println!("{}", some_or_none_type(Some(str)));
}
