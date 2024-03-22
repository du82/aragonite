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

#[get("/dw")]
pub fn dw() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/serps/dw.html"))
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

#[get("/jobs")]
pub fn jobs() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/jobs.html"))
}

#[get("/settings")]
pub fn settings() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/settings.html"))
}

// -----Themes-----

#[get("/themes/landing.css")]
pub fn landing_theme_route() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/landing.css"))
}

#[get("/themes/media/logo.png")]
pub fn logo_image() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/media/logo.png"))
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

#[get("/themes/dracula.css")]
pub fn dracula_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/dracula.css"))
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

#[get("/themes/r_theme.css")]
pub fn r_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/r_theme.css"))
}

#[get("/themes/stealth.css")]
pub fn stealth_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/stealth.css"))
}

// -----Theme Selector-----

#[get("/themes?<selection>")]
pub fn theme_selector(selection: Option<String>, mut cookies: Cookies) -> Result<Redirect, std::io::Error> {
    if let Some(theme) = selection {
        cookies.add(Cookie::new("theme", theme));
    }

    Ok(Redirect::to("/settings"))
}