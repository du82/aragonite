use rocket::{get, post, routes, State};
use rocket::request::{Form, FromForm};
use rocket::http::{Cookies, Cookie};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use std::collections::HashMap;
use rocket::response::Response;
use rocket::http::ContentType;
use std::io::Cursor;
use std::path::Path;
use std::sync::Mutex;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;


pub struct User {
    pub logged_in: Mutex<bool>,
}

#[derive(FromForm)]
pub struct LoginForm {
    username: String,
    password: String,
}

// Place the function here, outside of any other functions
fn generate_random_session_id() -> String {
    Uuid::new_v4().to_string()
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
pub fn login_post(login_form: Form<LoginForm>, mut cookies: Cookies, users_map: State<HashMap<String, (String, String)>>, sessions: State<Mutex<HashMap<String, String>>>) -> Redirect {
    // Print out the entered username and password and the users_map
    println!("Username: {}, Password: {}", login_form.username, login_form.password);
    println!("{:?}", *users_map);

    // Check if the password is between 3 and 64 characters long
    if login_form.password.len() < 3 || login_form.password.len() > 64 {
        return Redirect::to("/login"); // Password is too short or too long
    }

    if let Some((display_name, password)) = users_map.get(&login_form.username) {
        if password == &login_form.password {
            let session_id = generate_random_session_id();
            cookies.add(Cookie::new("user_id", session_id.clone()));
            sessions.lock().unwrap().insert(session_id, display_name.clone());
            return Redirect::to("/internal");
        }
    }
    Redirect::to("/login") // Incorrect username or password
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("user_id"));
    Redirect::to("/login")
}

#[get("/internal")]
pub fn internal(cookies: Cookies, sessions: State<Mutex<HashMap<String, String>>>) -> Result<Response<'static>, Redirect> {
    if let Some(cookie) = cookies.get("user_id") {
        let session_id = cookie.value();
        if let Some(display_name) = sessions.lock().unwrap().get(session_id) {
            let mut file = File::open("static/internal.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents = contents.replace("{{display_name}}", display_name);
            let response = Response::build()
                .header(ContentType::HTML)
                .sized_body(Cursor::new(contents))
                .finalize();
            Ok(response)
        } else {
            Err(Redirect::to("/login"))
        }
    } else {
        Err(Redirect::to("/login"))
    }
}