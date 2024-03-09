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

#[get("/obscure")]
pub fn obscure() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/obscure.html"))
}

#[get("/porn")]
pub fn porn() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/porn.html"))
}

#[get("/dw")]
pub fn dw() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/dw.html"))
}

#[get("/themes/light.css")]
pub fn light_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/light.css"))
}

#[get("/themes/dark.css")]
pub fn dark_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/dark.css"))
}

#[get("/themes/high-contrast.css")]
pub fn high_contrast_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/high-contrast.css"))
}

#[get("/themes?<selection>")]
pub fn theme_selector(selection: Option<String>, mut cookies: Cookies) -> Result<NamedFile, std::io::Error> {
    let theme = match selection {
        Some(theme) => theme,
        None => "light".to_string(),
    };

    let file_path = match theme.as_str() {
        "light" => {
            cookies.add(Cookie::new("theme", "light"));
            "themes/light.css"
        },
        "dark" => {
            cookies.add(Cookie::new("theme", "dark"));
            "themes/dark.css"
        },
        "high-contrast" => {
            cookies.add(Cookie::new("theme", "dark"));
            "themes/high-contrast.css"
        },
        _ => {
            cookies.add(Cookie::new("theme", "light"));
            "themes/light.css"
        },
    };

    NamedFile::open(Path::new(file_path))
}

pub fn current_theme(cookies: &Cookies) -> String {
    let theme_cookie = cookies.get("theme").map(|cookie| cookie.value().to_string());
    match theme_cookie {
        Some(theme) => theme,
        None => "light.css".to_string(),
    }
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