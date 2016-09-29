extern crate phrases;

//use phrases::english::{greetings,farewells};
use phrases::english;
use phrases::japanese;

fn main() {
    println!("Hello   in english  is {}", english::hello()   );
    println!("Goodbye in english  is {}", english::goodbye() );

    println!("Hello   in japanese is {}", japanese::hello()  );
    println!("Goodbye in japanese is {}", japanese::goodbye());
}
