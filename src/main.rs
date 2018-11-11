#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
#[macro_use] extern crate rocket;

mod login;

use rocket::Request;
use rocket::fairing::AdHoc;
use rocket_contrib::serve::{StaticFiles, Options};
use rocket_contrib::templates::Template;

use login::Login;

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Login Credentials", |rocket| {

            let valid_login = Login {
                username: rocket.config().get_str("username").expect("No Username").to_owned(),
                password: rocket.config().get_str("password").expect("No Password").to_owned(),
            };

            Ok(rocket.manage(valid_login))
        }))
        .mount("/", routes![login::login, login::login_page])
        .mount("/", StaticFiles::new("static", Options::Index))
        .register(catchers![not_found])
        .launch();
}