#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

mod index;
mod page;

extern crate rocket;
//extern crate rustache;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

fn main() {
    index::rocket().launch();
}
