#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
#[macro_use] extern crate rocket;
use rocket::Request;
use rocket_contrib::serve::{StaticFiles, Options};
use rocket_contrib::templates::Template;

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", StaticFiles::new("static", Options::Index))
        .register(catchers![not_found])
        .launch();
}