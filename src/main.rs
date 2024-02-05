#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, post, routes, State};
use rocket::request::{Form, FromForm};
use rocket::http::{Cookies, Cookie};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use std::path::Path;
use std::sync::Mutex;

struct User {
    logged_in: Mutex<bool>,
}

#[derive(FromForm)]
struct Login {
    password: String,
}

#[get("/")]
fn index(cookies: Cookies) -> Result<Redirect, &'static str> {
    match cookies.get("user_id") {
        Some(_) => Ok(Redirect::to("/internal")),
        None => Ok(Redirect::to("/login")),
    }
}

#[get("/login")]
fn login(cookies: Cookies) -> Result<NamedFile, Redirect> {
    if cookies.get("user_id").is_some() {
        Err(Redirect::to("/internal"))
    } else {
        Ok(NamedFile::open(Path::new("static/login.html")).unwrap())
    }
}

#[post("/login", data = "<login_form>")]
fn login_post(login_form: Form<Login>, mut cookies: Cookies, user: State<User>) -> Redirect {
    if login_form.password == "cats" {
        cookies.add(Cookie::new("user_id", "1"));
        let mut logged_in = user.logged_in.lock().unwrap();
        *logged_in = true;
        Redirect::to("/internal")
    } else {
        Redirect::to("/login")
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("user_id"));
    Redirect::to("/login")
}

#[get("/internal")]
fn internal(cookies: Cookies) -> Result<NamedFile, Redirect> {
    if cookies.get("user_id").is_some() {
        Ok(NamedFile::open(Path::new("static/internal.html")).unwrap())
    } else {
        Err(Redirect::to("/login"))
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, login, login_post, logout, internal])
        .manage(User { logged_in: Mutex::new(false) })
        .launch();
}