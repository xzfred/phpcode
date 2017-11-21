use rocket::{self, Request, Route, Data, Catcher, Error};
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::handler::Outcome;
use rocket::http::Method::*;
use rocket::http::{Status, RawStr};

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>
}

fn forward(req: &Request, data: Data) -> Outcome<'static> {
    //Outcome::forward(data)
    let param = req.uri()
        .as_str()
        //.split_at(6)
        //.1
        ;

    Outcome::from(req, RawStr::from_str(param).url_decode())
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/hello/Unknown")
}

#[get("/hello/<name>", rank = 1)]
fn get(name: String) -> Template {
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect()
    };

    Template::render("index", &context)
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

pub fn rocket() -> rocket::Rocket {
    //let always_forward = Route::ranked(1, Get, "/", forward);
    rocket::ignite()
        .mount("/", routes![index, get, files])
        //.mount("/", vec![always_forward])
        //.mount("/hello", routes![get])
        .attach(Template::fairing())
        .catch(errors![not_found])
}
