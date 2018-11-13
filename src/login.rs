use std::collections::HashMap;

use bcrypt::verify;
use rocket::fairing::AdHoc;
use rocket::request::{Form, FlashMessage};
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
pub fn login(mut cookies: Cookies, login: Form<Login>, valid_login: State<(String, String)>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if (*login.username == (*valid_login).0) &
        (verify(&login.password, &(*valid_login).1).expect("Could not hash password"))
    {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Flash::success(Redirect::to(uri!(login_page)), "Correct username/password."))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    }
}

#[delete("/login")]
pub fn logout(mut cookies:Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out")
}

pub fn get_valid_login() -> AdHoc {
    AdHoc::on_attach("Login Credentials", |rocket| {

        let valid_login = (
            rocket.config().get_str("username").expect("No Username").to_owned(),
            rocket.config().get_str("passhash").expect("No Passhash").to_owned(),
        );

        Ok(rocket.manage(valid_login))
    })
}