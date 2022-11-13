#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, Options};
use rocket::Request;
use rocket_dyn_templates::{
    context,
    handlebars::{self, JsonRender},
    Template,
};

// mod blog;
#[cfg(test)]
mod test;

#[get("/health_check")]
fn health_check() {}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("error/404", context! { path: req.uri().path().as_str() })
}

fn markdown_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

#[launch]
fn rocket() -> _ {
    eprintln!("Preparing for liftoff...");
    rocket::build()
        //.attach(blog::ContentDb::fairing())
        .attach(Template::custom(|engine| {
            engine
                .handlebars
                .register_helper("markdown", Box::new(markdown_helper));
        }))
        .mount("/", routes![health_check])
        .mount("/", FileServer::new("static", Options::Index))
        // .mount("/", routes![blog::blog, blog::latest_blog])
        .register("/", catchers![not_found])
}
