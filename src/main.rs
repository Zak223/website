#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::env;
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
        .secret_key(&env::var("SECRET_KEY").unwrap())
        .unwrap();

    rocket::custom(cfg)
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
