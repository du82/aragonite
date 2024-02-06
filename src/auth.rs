use rocket::{get, post, routes, State};
use rocket::request::{Form, FromForm};
use rocket::http::{Cookies, Cookie};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

// Your function definitions go here

pub struct User {
    pub logged_in: Mutex<bool>,
}

#[derive(FromForm)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[get("/")]
pub fn index(cookies: Cookies) -> Result<Redirect, &'static str> {
    match cookies.get("user_id") {
        Some(_) => Ok(Redirect::to("/internal")),
        None => Ok(Redirect::to("/login")),
    }
}

#[get("/login")]
pub fn login(cookies: Cookies) -> Result<NamedFile, Redirect> {
    if cookies.get("user_id").is_some() {
        Err(Redirect::to("/internal"))
    } else {
        Ok(NamedFile::open(Path::new("static/login.html")).unwrap())
    }
}

#[post("/login", data = "<login_form>")]
pub fn login_post(login_form: Form<LoginForm>, mut cookies: Cookies, users_map: State<HashMap<String, String>>) -> Redirect {
    if let Some(password) = users_map.get(&login_form.username) {
        if password == &login_form.password {
            cookies.add(Cookie::new("user_id", login_form.username.clone()));
            Redirect::to("/internal")
        } else {
            Redirect::to("/login") // Incorrect password
        }
    } else {
        Redirect::to("/login") // Username not found
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("user_id"));
    Redirect::to("/login")
}

#[get("/internal")]
pub fn internal(cookies: Cookies) -> Result<NamedFile, Redirect> {
    if cookies.get("user_id").is_some() {
        Ok(NamedFile::open(Path::new("static/internal.html")).unwrap())
    } else {
        Err(Redirect::to("/login"))
    }
}