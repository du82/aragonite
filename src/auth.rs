use rocket::get;
use rocket::http::{Cookies, Cookie};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use std::path::Path;

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

#[get("/donate")]
pub fn donate() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/donate.html"))
}

#[get("/themes/light.css")]
pub fn light_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/light.css"))
}

#[get("/themes/dark.css")]
pub fn dark_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/dark.css"))
}

#[get("/themes/graphite.css")]
pub fn graphite_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/graphite.css"))
}

#[get("/themes/high-contrast.css")]
pub fn high_contrast_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/high-contrast.css"))
}

#[get("/themes/midnight.css")]
pub fn midnight_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/midnight.css"))
}

#[get("/themes/clown.css")]
pub fn clown_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/clown.css"))
}

#[get("/themes/420.css")]
pub fn weed_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/420.css"))
}

#[get("/themes/monokai.css")]
pub fn monokai_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/monokai.css"))
}


#[get("/themes/ristretto.css")]
pub fn ristretto_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/ristretto.css"))
}

#[get("/themes?<selection>")]
pub fn theme_selector(selection: Option<String>, mut cookies: Cookies) -> Result<Redirect, std::io::Error> {
    if let Some(theme) = selection {
        cookies.add(Cookie::new("theme", theme));
    }

    // Redirect to the home page or another appropriate page after setting the theme
    Ok(Redirect::to("/"))
}

pub fn current_theme(cookies: &Cookies) -> String {
    let theme_cookie = cookies.get("theme").map(|cookie| cookie.value().to_string());
    match theme_cookie {
        Some(theme) => theme,
        None => "light.css".to_string(),
    }
}