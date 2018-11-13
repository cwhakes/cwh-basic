#![feature(proc_macro_hygiene, decl_macro)]

extern crate bcrypt;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod login;

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
        .attach(login::get_valid_login())
        .mount("/", routes![login::login_page, login::login, login::logout])
        .mount("/", StaticFiles::new("static", Options::Index))
        .register(catchers![not_found])
        .launch();
}