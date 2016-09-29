fn takes_a_slice(str_slice: &str) {
    println!("Got {}", str_slice);
}

//fn duderize(str_slice: String) {
    //str_slice.push_str(" is a cool dude!");
//}

fn main() {
    let s1 = "Hello There.";          // s1: &str       *** statically allocated string 
    let mut s2 = "Hello".to_string(); // mut s2: String *** in memory string
    //let mut s3 = "Cliff";

    println!("{}", s2);
    s2.push_str(", dude");
    println!("{}", s2); // see, it can change!

    takes_a_slice(s1);

    //duderize(s3);

    //println!("{}", s3);
}
