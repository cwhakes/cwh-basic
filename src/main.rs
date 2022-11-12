#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer, Options};
use rocket::Request;
use rocket_dyn_templates::{
    context,
    handlebars::{self, JsonRender},
    Template,
};

// mod blog;

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
    rocket::build()
        //.attach(blog::ContentDb::fairing())
        .attach(Template::custom(|engine| {
            engine
                .handlebars
                .register_helper("markdown", Box::new(markdown_helper));
        }))
        .mount("/", FileServer::new(relative!("static"), Options::Index))
        // .mount("/", routes![blog::blog, blog::latest_blog])
        .register("/", catchers![not_found])
}
