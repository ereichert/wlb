extern crate env_logger;
extern crate iron;
extern crate router;

use iron::prelude::{Chain, Iron, Request, Response};
use iron::{IronResult, status, Set};
extern crate handlebars_iron as hbs;
use hbs::{HandlebarsEngine};
#[cfg(feature = "watch")]
use hbs::Watchable;
use std::sync::Arc;
use std::error::Error;

fn main() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(hbs::DirectorySource::new("./src/views/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{:?}", r.description());
    }
    let hbse_ref = Arc::new(hbse);
    hbse_ref.watch("./src/views/");

    let mut home_chain = Chain::new(home_handler);
    home_chain.link_after(hbse_ref);

    let mut router = router::Router::new();
    router.get("/", home_chain, "home");

    let port = 3000;
    let bind_addr = format!("localhost:{}", port);
    let _server_guard = Iron::new(router).http(bind_addr.as_str()).unwrap();

    let version = include_str!("version.txt");
    println!("Running WLB v{} on port {}.", version, port)
}


fn home_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(hbs::Template::new("home", "".to_string())).set_mut(status::Ok);
    Ok(resp)
}