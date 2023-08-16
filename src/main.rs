#![allow(dead_code)]

mod http;
mod handler;
mod server;

use server::Server;
use crate::handler::WebHandler;

fn main() {
    // server is a Struct
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebHandler::new());
}
