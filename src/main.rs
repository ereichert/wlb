extern crate env_logger;
extern crate iron;

use iron::prelude::{Chain, Iron, Request, Response};
use iron::{Set,status};
extern crate handlebars_iron as hbs;
use std::error::Error;

fn main() {
    env_logger::init().unwrap();

    let mut hbse = hbs::HandlebarsEngine::new();
    hbse.add(Box::new(hbs::DirectorySource::new("./src/views/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{:?}", r.description());
    }

    let mut chain = Chain::new(|_: &mut Request| {
        let mut resp = Response::new();
        resp.set_mut(hbs::Template::new("index", "".to_string())).set_mut(status::Ok);
        Ok(resp)
    });

    chain.link_after(hbse);

    let port = 3000;
    let bind_addr = format!("localhost:{}", port);
    let _server_guard = Iron::new(chain).http(bind_addr.as_str()).unwrap();

    let version = include_str!("version.txt");
    println!("Running WLB v{} on port {}.", version, port)
}