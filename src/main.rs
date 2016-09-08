extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    let port = 3000;
    let bind_addr = format!("localhost:{}", port);
    let _server_guard = Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http(bind_addr.as_str()).unwrap();

    let version = include_str!("version.txt");
    println!("Running WLB v{} on port {}.", version, port)
}

