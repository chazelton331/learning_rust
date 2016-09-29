use std::io;

fn main() {
    println!("here we go!");

    loop {
        let mut s = String::new();

        io::stdin().read_line(&mut s).expect("failed to read!");

        let mut sp          = s.split(" ");
        let vec: Vec<&str>  = sp.collect();

        for v in &vec {
            println!("{}", v);
        }
    }
}
