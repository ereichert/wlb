extern crate env_logger;
extern crate iron;
extern crate handlebars_iron as hbs;
extern crate router;
extern crate mount;
extern crate staticfile;

use iron::prelude::{Chain, Iron, Request, Response};
use iron::{IronResult, status, Set};

use hbs::{HandlebarsEngine};
#[cfg(feature = "watch")]
use hbs::Watchable;

use mount::Mount;
use staticfile::Static;

use std::sync::Arc;
use std::error::Error;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let views_ext = ".hbs";
    let views_path = "./src/views";

    let mut hbse = HandlebarsEngine::new();
    // TODO: Investigate serving the templates out of the binary using include_string!
    hbse.add(Box::new(hbs::DirectorySource::new(views_path, views_ext)));
    if let Err(r) = hbse.reload() {
        panic!("{:?}", r.description());
    }
    let hbse_ref = Arc::new(hbse);
    hbse_ref.watch(views_path);

    let mut home_chain = Chain::new(home_handler);
    home_chain.link_after(hbse_ref);

    let mut router = router::Router::new();
    router.get("/", home_chain, "get_home");

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

fn home_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(hbs::Template::new("home", "".to_string())).set_mut(status::Ok);
    Ok(resp)
}