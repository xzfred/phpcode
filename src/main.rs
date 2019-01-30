#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix_web::{middleware, server, App, HttpRequest, HttpResponse};

/// simple handle
fn index(req: &HttpRequest) -> HttpResponse {
    // println!("{:?}", req);
    dbg!(req);

    HttpResponse::Ok().body(format!("Num of requests: {}", "aa"))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("ws-example");

    server::new(|| {
        App::new() // <- create app with shared state
        // enable logger
            .middleware(middleware::Logger::default())
        // register simple handler, handle all methods
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
