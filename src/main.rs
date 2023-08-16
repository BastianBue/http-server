#![allow(dead_code)]

mod http;
mod handler;
mod server;

use std::env;
use server::Server;
use crate::handler::WebHandler;

fn main() {
    // server is a Struct
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebHandler::new(public_path));
}
