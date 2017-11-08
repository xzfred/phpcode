#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

mod index;
mod page;

extern crate rocket;

fn main() {
    index::rocket().launch();
}
