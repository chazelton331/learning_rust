extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

struct Charge {
}

fn main() {
    let mut client = Client::new();

    let mut result = client.get("https://something.example.com/service/v1/example.json")
                            .header(Connection::close())
                            .send()
                            .unwrap();

    let mut body = String::new();

    result.read_to_string(&mut body).unwrap();

    println!("Got: {}", body);
}
