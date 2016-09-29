extern crate hyper;
extern crate serde;
extern crate serde_json;


use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::header::Headers;
use hyper::header::Server as ServerHeader;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};


struct PingResponse {
    success: bool,
    message: String,
}

impl serde::Serialize for PingResponse {

    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_struct("PingResponse", PingResponseMapVisitor {
            value: self,
            state: 0,
        })
    }

}

struct PingResponseMapVisitor<'a> {
    value: &'a PingResponse,
    state: u8,
}

impl<'a> serde::ser::MapVisitor for PingResponseMapVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: serde::Serializer
    {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("success", &self.value.success))))
            }
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("message", &self.value.message))))
            }
            _ => {
                Ok(None)
            }
        }
    }
}


fn handle_root(_: Request, res: Response) {
    let mut headers = Headers::new();

    headers.set(ServerHeader("hyper/0.8.0".to_owned()));

    headers.set(
        ContentType(
            Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])
        )
    );

    let response_message     = "It worked!".to_string();
    let ping_resp            = PingResponse { success: true, message: response_message };
    let serialized_ping_resp = serde_json::to_string(&ping_resp).unwrap();

    res.send(serialized_ping_resp.as_bytes()).unwrap(); /* here need serialized_ping_resp to be binary */
}

fn main() {
    //let host        = "127.0.0.1".to_string();
    //let port        = "43110";
    //let hostport    = host + ":" + port;

    //println!("Started on host={} port={}", host, port);

    //Server::http(hostport).unwrap().handle(handle_root).unwrap();
    Server::http("127.0.0.1:43110").unwrap().handle(handle_root).unwrap();
}
