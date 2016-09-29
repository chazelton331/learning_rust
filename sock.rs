use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

fn handle_client(mut tcp_stream: TcpStream) {
    println!("new client joined");

    loop {
        let write_bytes     = String::from("x> ").into_bytes();
        let _               = tcp_stream.write(&write_bytes);

        let mut read_bytes  = String::new();
        let response        = tcp_stream.read_to_string(&mut read_bytes);

        println!("{}", read_bytes);

        break;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:43110").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }

            Err(e) => {
                println!("connection failed");
            }
        }
    }

    drop(listener);
}
