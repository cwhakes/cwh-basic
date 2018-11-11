use std::collections::HashMap;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(FromForm, PartialEq, Eq)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[get("/login")]
pub fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)    
}


#[post("/login", data = "<login>")]
pub fn login(mut cookies: Cookies, login: Form<Login>, valid_login: State<Login>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if *login == *valid_login {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Flash::success(Redirect::to(uri!(login_page)), "Correct username/password."))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    }
}