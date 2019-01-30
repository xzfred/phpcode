#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

mod index;
mod page;
mod base;

extern crate rocket;
//extern crate rustache;
extern crate rocket_contrib;
#[macro_use] 
extern crate serde_derive;
extern crate mysql;

fn main() {
    let cfg = base::config::Config::new();
    let my = base::db::MyPool::new(&cfg.database);
    my.new_user();

    index::rocket().launch();
}
