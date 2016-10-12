#![feature(custom_derive, plugin)]
#![plugin(tojson_macros)]

extern crate env_logger;
extern crate iron;
extern crate handlebars as hbs;
extern crate handlebars_iron as hbi;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate urlencoded;
extern crate rustc_serialize;

use std::sync::Arc;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;

use iron::prelude::{Chain, Iron, Request, Response};
use iron::{IronResult, status, Set};
use iron::Plugin;
use hbs::Handlebars;
use hbi::{HandlebarsEngine};
use mount::Mount;
use staticfile::Static;
use urlencoded::UrlEncodedBody;

mod view_helpers;
mod routing;
mod handlers;

fn main() {
    env_logger::init().unwrap();

    let views_ext = ".hbs";
    let views_path = "./src/views";

    let mut hbs = Handlebars::new();
    hbs.register_helper("if-multiple-of", Box::new(view_helpers::if_multiple_of_helper));
    let mut hbse = HandlebarsEngine::from(hbs);
    // TODO: Investigate serving the templates out of the binary using include_string!
    hbse.add(Box::new(hbi::DirectorySource::new(views_path, views_ext)));
    if let Err(r) = hbse.reload() {
        panic!("{:?}", r.description());
    }
    let hbse_ref = Arc::new(hbse);
    if cfg!(debug_assertions) {
        println!("WARNING: DEBUG ASSERTIONS ENABLED.  TEMPLATES ARE WATCHED.");
        use hbi::Watchable;
        hbse_ref.watch(views_path);
    }

    let mut home_chain = Chain::new(handlers::home_handler);
    home_chain.link_after(hbse_ref);

    let mut router = router::Router::new();
    router.get("/", home_chain, "get_home");
    router.post("/new_task", new_task_handler, "new_task");

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/assets/", Static::new(Path::new("src/assets")));

    let port = 3000;
    let bind_addr = format!("localhost:{}", port);
    let _server_guard = Iron::new(assets_mount).http(bind_addr.as_str()).unwrap();

    let version = include_str!("version.txt");
    println!("Running WLB v{} on port {}.", version, port)
}

fn new_task_handler(req: &mut Request) -> IronResult<Response> {
    println!("Reached the new task handler");
    println!("req = {:?}", req);
    let empty_hm = HashMap::new();
    let hm = match req.get_ref::<UrlEncodedBody>() {
        Ok(hashmap) => hashmap,
        Err(ref e) => {
            println!("{:?}", e);
            &empty_hm
        }
    };


    println!("encoded data = {:?}", hm);
    let mut resp = Response::new();
    resp.set_mut(hbi::Template::new("home", handlers::make_test_records())).set_mut(status::Ok);
    Ok(resp)
}