#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![allow(dead_code)]
#![allow(unused_mut)]

extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate json;
extern crate proclist;

mod sql_builder;

use sql_builder::insert::Insert;

use actix_web::{middleware, server, App, HttpRequest, HttpResponse, fs};

/// simple handle
fn index(req: &HttpRequest) -> HttpResponse {
    // println!("{:?}", req);
    dbg!(req);

    HttpResponse::Ok().body(format!("Num of requests: {}", "aa"))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    // json_test();

    dbg!(Insert::new().to_string());
    dbg!(Insert::new());
}

fn web() {
    let sys = actix::System::new("ws-example");
    server::new(|| {
        App::new() // <- create app with shared state
        // enable logger
            .middleware(middleware::Logger::default())
        // 静态文件
            .handler("/www", fs::StaticFiles::new("www/").unwrap().show_files_listing())
        // register simple handler, handle all methods
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}

fn json_test() {
    let mut data = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42
    };
    dbg!(&data);
    data["add_data"] = object!{
        "foo" => json::Null,
    };
    println!("{}", json::stringify_pretty(data, 4));
}
