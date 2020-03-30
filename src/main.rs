#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::get;
use rocket_contrib;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::config::{Config, Environment};
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}


fn main() {
    let cfg = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8081)
        .secret_key("VpL1gPPTPBStZsKLQq4tDJITTu5pkjowWmqrMZVk++8=")
        .unwrap();

    rocket::custom(cfg)
        .mount("/", routes![index, contact, about, apply])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
