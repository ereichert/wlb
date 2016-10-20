#![feature(custom_derive, plugin)]
#![plugin(tojson_macros)]

extern crate env_logger;
extern crate iron;
extern crate handlebars as hbs;
extern crate handlebars_iron as hbi;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate urlencoded;

use std::sync::Arc;
use std::error::Error;
use std::path::Path;

use iron::prelude::{Chain, Iron};
use hbs::Handlebars;
use hbi::{HandlebarsEngine};
use mount::Mount;
use staticfile::Static;

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

    let mut router = chain::chain();
    let mut assets_mount = Mount::new();
    assets_mount.mount("/assets/", Static::new(Path::new("src/assets")));
    router.get("/assets/*", assets_mount, "assets");
    let mut home_chain = Chain::new(router);
    home_chain.link_after(hbse_ref);

    let port = 3000;
    let bind_addr = format!("localhost:{}", port);
    let _server_guard = Iron::new(home_chain).http(bind_addr.as_str()).unwrap();

    let version = include_str!("version.txt");
    println!("Running WLB v{} on port {}.", version, port)
}