#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/<name>/<age>")]
fn hello_who(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    //rocket::ignite().mount("/", routes![hello]).launch();
    rocket::ignite().mount("/hello", routes![hello_who]).launch();
}
