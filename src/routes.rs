use rocket::get;
use rocket::http::{Cookies, Cookie};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use std::path::Path;

// -----Routes-----

#[get("/")]
pub fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/landing.html"))
}

#[get("/favicon.ico")]
pub fn favicon() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("favicon.ico"))
}

#[get("/donate")]
pub fn donate() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/donate.html"))
}

#[get("/about")]
pub fn about() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/about.html"))
}

#[get("/privacy")]
pub fn privacy() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/privacy.html"))
}

#[get("/tos")]
pub fn tos() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/tos.html"))
}

#[get("/mirrors")]
pub fn mirrors() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/mirrors.html"))
}

#[get("/settings")]
pub fn settings() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/settings.html"))
}

// -----Themes-----

#[get("/themes/media/logo.png")]
pub fn logo_image() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/media/logo.png"))
}

#[get("/themes/gemstone.css")]
pub fn gemstone_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/gemstone.css"))
}

#[get("/themes/gemstone_light.css")]
pub fn gemstone_light_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/gemstone_light.css"))
}

#[get("/themes/high-contrast.css")]
pub fn high_contrast_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/high-contrast.css"))
}

#[get("/themes/midnight.css")]
pub fn midnight_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/midnight.css"))
}

// -----Theme Selector-----

#[get("/themes?<selection>")]
pub fn theme_selector(selection: Option<String>, mut cookies: Cookies) -> Result<Redirect, std::io::Error> {
    if let Some(theme) = selection {
        cookies.add(Cookie::new("theme", theme));
    }

    Ok(Redirect::to("/settings"))
}