
extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rustc_serialize::json;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");

    println!("Listening on localhost:3009");
    Iron::new(router).http("localhost:3009").ok();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let out = "{\"status\": \"OK\"}\n\n\r\n\r\n";

    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, out)))
}
