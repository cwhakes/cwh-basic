#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
extern crate chrono;
extern crate handlebars_markdown_helper;
use rocket::Request;
use rocket_contrib::serve::{StaticFiles, Options};
use rocket_contrib::templates::Template;
use handlebars_markdown_helper::markdown_helper;

mod blog;

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .attach(blog::ContentDb::fairing())
        .attach(Template::custom(|engine| {
            engine.handlebars.register_helper(
                "markdown", Box::new(markdown_helper)
            );
        }))
        .mount("/", StaticFiles::new("static", Options::Index))
        .mount("/", routes![blog::blog])
        .register(catchers![not_found])
        .launch();
}