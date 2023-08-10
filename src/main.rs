#![allow(dead_code)]
// pull a module into scope
mod http;
mod server;

// pull method from modules into scope
use server::Server;

fn main() {
    // server is a Struct
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
