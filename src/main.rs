#![feature(custom_derive, plugin)]
#![feature(proc_macro)]

extern crate env_logger;
extern crate iron;
extern crate handlebars as hbs;
extern crate handlebars_iron as hbi;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate urlencoded;
#[macro_use]
extern crate tojson_macros;

use std::sync::Arc;
use std::error::Error;

use iron::prelude::Iron;
use hbs::Handlebars;
use hbi::{HandlebarsEngine};

mod view_helpers;
mod chain;
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

    let mut chain = chain::chain();
    chain.link_after(hbse_ref);

    let port = 3000;
    let bind_addr = format!("localhost:{}", port);
    let _server_guard = Iron::new(chain).http(bind_addr.as_str()).unwrap();

    let version = include_str!("version.txt");
    println!("Running WLB v{} on port {}.", version, port)
}