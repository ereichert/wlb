use iron::prelude::{Request, Response};
use iron::{IronResult, Set, status};
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use hbi;

pub fn home_handler(_: &mut Request) -> IronResult<Response> {
    let data = make_test_records();
    let mut resp = Response::new();
    resp.set_mut(hbi::Template::new("home", data)).set_mut(status::Ok);
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

pub fn make_test_records() -> BTreeMap<String, Vec<Json>> {
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