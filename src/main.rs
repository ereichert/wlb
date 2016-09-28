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

use iron::prelude::{Chain, Iron, Request, Response};
use iron::{IronResult, status, Set};

use hbs::{Handlebars, RenderError, RenderContext, Helper, Context, Renderable};
use hbi::{HandlebarsEngine};
#[cfg(feature = "watch")]
use hbi::Watchable;

use mount::Mount;
use staticfile::Static;

use rustc_serialize::json::{Json, ToJson};

use std::sync::Arc;
use std::error::Error;
use std::path::Path;
use std::collections::BTreeMap;

fn main() {
    env_logger::init().unwrap();

    let views_ext = ".hbs";
    let views_path = "./src/views";

    let mut hbs = Handlebars::new();
    hbs.register_helper("if-multiple-of", Box::new(if_multiple_of_helper));
    let mut hbse = HandlebarsEngine::from(hbs);
    // TODO: Investigate serving the templates out of the binary using include_string!
    hbse.add(Box::new(hbi::DirectorySource::new(views_path, views_ext)));
    if let Err(r) = hbse.reload() {
        panic!("{:?}", r.description());
    }

    // TODO: Put this behind a feature flag.
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
    let data = make_test_records();
    let mut resp = Response::new();
    resp.set_mut(hbi::Template::new("home", data)).set_mut(status::Ok);
    Ok(resp)
}

const FACTOR_OF_INTEREST_IDX: usize = 0;
const CANDIDATE_IDX: usize = 1;
fn if_multiple_of_helper(ctx: &Context, helper: &Helper, hbars: &Handlebars, render_ctx: &mut RenderContext) -> Result<(), RenderError> {
    let factor_of_interest = try!(
        helper.param(FACTOR_OF_INTEREST_IDX)
            .map(|json| json.value())
            .and_then(|val| val.as_u64())
            .and_then(|u64_val| if u64_val > 0 { Some(u64_val) } else { None } )
            .ok_or_else(|| RenderError::new("Factor of interest must be a number greater than 0."))
    );

    let candidate = try!(
        helper.param(CANDIDATE_IDX)
            .map(|json| json.value())
            .and_then(|val| val.as_u64())
            .ok_or_else(|| RenderError::new("Candidate must be a number greater than or equal to 0."))
    );

    let possible_template = if candidate % factor_of_interest == 0 {
        helper.template()
    } else {
        helper.inverse()
    };

    match possible_template {
        Some(t) => t.render(ctx, hbars, render_ctx),
        None => Ok(()),
    }
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

#[cfg(test)]
mod if_multiple_of_helper_tests {
    extern crate handlebars as hbs;

    use hbs::{Handlebars, Template};

    #[derive(ToJson)]
    struct TestCandidate {
        value: i64
    }

    #[test]
    fn should_return_an_err_when_one_of_the_parameters_is_missing() {
        let template = Template::compile(
            "{{#if-multiple-of 2}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 2});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_an_err_when_the_candidate_is_less_than_0() {
        let template = Template::compile(
            "{{#if-multiple-of 2 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: -3});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_an_err_when_the_factor_of_interest_is_0() {
        let template = Template::compile(
            "{{#if-multiple-of 0 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 3});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_an_err_when_the_factor_of_interest_is_less_than_0() {
        let template = Template::compile(
            "{{#if-multiple-of -1 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 3});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_the_is_not_multiple_template_when_the_candidate_is_a_multiple_of_the_factor() {
        let template = Template::compile(
            "{{#if-multiple-of 2 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 3});

        assert_eq!("IS_NOT_MULTIPLE", rendered.ok().unwrap());
    }

    #[test]
    fn should_return_the_is_multiple_template_when_the_candidate_is_a_multiple_of_the_factor() {
        let template = Template::compile(
            "{{#if-multiple-of 2 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 2});

        assert_eq!("IS_MULTIPLE", rendered.ok().unwrap());
    }
}