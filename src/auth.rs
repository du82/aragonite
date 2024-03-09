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
pub fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/landing.html"))
}

#[get("/images")]
pub fn images() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/images.html"))
}

#[get("/videos")]
pub fn videos() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/videos.html"))
}

#[get("/news")]
pub fn news() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/news.html"))
}

#[get("/porn")]
pub fn porn() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/porn.html"))
}

#[get("/dw")]
pub fn dw() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/dw.html"))
}

#[get("/main.css")]
pub fn css_styling() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/main.css"))
}

#[get("/internal")]
pub fn internal(cookies: Cookies, sessions: State<Mutex<HashMap<String, String>>>) -> Result<Response<'static>, Redirect> {
    if let Some(cookie) = cookies.get("user_id") {
        let session_id = cookie.value();
        if let Some(display_name) = sessions.lock().unwrap().get(session_id) {
            let mut file = File::open("static/serps/web.html").unwrap();
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