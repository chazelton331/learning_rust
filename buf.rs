fn get_file_string(path_str: &String) -> String{
    let path = Path::new(path_str.as_bytes());
    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);
    reader.read_to_string().unwrap()
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8001");
    let mut acceptor = listener.listen();

    let ref file_to_host = os::args()[1];
    let html = get_file_string(file_to_host).clone();

    fn handle_client(mut stream: TcpStream, html: String) {
        let write = stream.write_str(html.as_slice());
    }

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!("{}", e) }
            Ok(stream) => spawn(proc() {
                handle_client(stream,html)
            })
        }
    }
}
