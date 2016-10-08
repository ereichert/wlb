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
use std::collections::{BTreeMap, HashMap};

use iron::prelude::{Chain, Iron, Request, Response};
use iron::{IronResult, status, Set};
use iron::Plugin;
use hbs::Handlebars;
use hbi::{HandlebarsEngine};
use mount::Mount;
use staticfile::Static;
use urlencoded::UrlEncodedBody;
use rustc_serialize::json::{Json, ToJson};

mod view_helpers;

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

    let mut home_chain = Chain::new(home_handler);
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

fn home_handler(_: &mut Request) -> IronResult<Response> {
    let data = make_test_records();
    let mut resp = Response::new();
    resp.set_mut(hbi::Template::new("home", data)).set_mut(status::Ok);
    Ok(resp)
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
    resp.set_mut(hbi::Template::new("home", make_test_records())).set_mut(status::Ok);
    Ok(resp)
}

#[derive(ToJson)]
struct TaskRecord<'a> {
    date: &'a str,
    start_time: &'a str,
    end_time: &'a str,
    project_name: &'a str,
    description: &'a str,
}

fn make_test_records() -> BTreeMap<String, Vec<Json>> {
    let mut data = BTreeMap::new();
    data.insert("task_records".to_string(), vec![
        TaskRecord {
            date: "09/19/2016",
            start_time: "06:58",
            end_time: "07:45",
            project_name: "TL",
            description: "Research, reading, schedule",
        }.to_json(),
        TaskRecord {
            date: "09/19/2016",
            start_time: "07:45",
            end_time: "09:30",
            project_name: "TL",
            description: "Development",
        }.to_json(),
        TaskRecord {
            date: "09/19/2016",
            start_time: "09:30",
            end_time: "09:34",
            project_name: "TL",
            description: "standup",
        }.to_json(),
        TaskRecord {
            date: "09/19/2016",
            start_time: "09:34",
            end_time: "09:45",
            project_name: "TL",
            description: "comms",
        }.to_json(),
        TaskRecord {
            date: "09/19/2016",
            start_time: "09:45",
            end_time: "14:00",
            project_name: "TL",
            description: "development",
        }.to_json(),
        TaskRecord {
            date: "09/19/2016",
            start_time: "14:30",
            end_time: "15:30",
            project_name: "TL",
            description: "aijsdf;lkjsdf;lkjs;flkja;lkfj;aslkfj;alksjf;laksjf;lkasjf;lkajsf;lkjsaflkjasflkjas;fkja;lkn;alvi;alkn;flkansf;lkjasdf;lkasd;flknq;oifj;qowif;okns;kan;sdkfna;lskfdnm;alkfn;kasfv;lknsmf;lkasf;lkjf;lkajfd;lkanmsf;lkajsf;lkansf;dlkajsf;lkam;lkansd;flkanf",
        }.to_json(),
    ]);
    data
}