// Core route handlers

use rocket::get;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::fs::NamedFile;
use rocket::State;
use std::path::Path;
use std::sync::Mutex;
use crate::filter::FilterManager;

#[get("/")]
pub async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/landing.html")).await
}

#[get("/favicon.ico")]
pub async fn favicon() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("favicon.ico")).await
}

#[get("/donate")]
pub async fn donate() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/donate.html")).await
}

#[get("/about")]
pub async fn about() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/about.html")).await
}

#[get("/privacy")]
pub async fn privacy() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/privacy.html")).await
}

#[get("/tos")]
pub async fn tos() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/tos.html")).await
}

#[get("/mirrors")]
pub async fn mirrors() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/mirrors.html")).await
}

#[get("/settings")]
pub async fn settings() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/settings.html")).await
}

// Safe Search routes

#[get("/safe_search?<enabled>")]
pub fn safe_search_toggle(enabled: bool, cookies: &CookieJar<'_>, filter_manager: &State<Mutex<FilterManager>>) -> Result<Redirect, std::io::Error> {
    cookies.add(Cookie::new("safe_search_enabled", enabled.to_string()));

    let mut filter_manager = filter_manager.lock().unwrap();
    filter_manager.enable(enabled);

    Ok(Redirect::to("/settings"))
}

#[get("/safe_search_level?<level>")]
pub fn safe_search_level(level: String, cookies: &CookieJar<'_>, filter_manager: &State<Mutex<FilterManager>>) -> Result<Redirect, std::io::Error> {
    let valid_level = match level.as_str() {
        "off" | "moderate" | "strict" => level.clone(),
        _ => "moderate".to_string(), // Default to moderate if invalid
    };

    cookies.add(Cookie::new("safe_search_level", valid_level.clone()));

    let mut filter_manager = filter_manager.lock().unwrap();
    filter_manager.set_level(&valid_level);

    Ok(Redirect::to("/settings"))
}

#[get("/syntax")]
pub async fn syntax() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/pages/syntax.html")).await
}

// Theme routes

#[get("/themes/media/logo.png")]
pub async fn logo_image() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/media/logo.png")).await
}

#[get("/themes/gemstone.css")]
pub async fn gemstone_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/gemstone.css")).await
}

#[get("/themes/gemstone_light.css")]
pub async fn gemstone_light_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/gemstone_light.css")).await
}

#[get("/themes/high-contrast.css")]
pub async fn high_contrast_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/high-contrast.css")).await
}

#[get("/themes/midnight.css")]
pub async fn midnight_theme() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/themes/midnight.css")).await
}

// Theme selection handler

#[get("/themes?<selection>")]
pub async fn theme_selector(selection: Option<String>, cookies: &CookieJar<'_>) -> Result<Redirect, std::io::Error> {
    if let Some(theme) = selection {
        cookies.add(Cookie::new("theme", theme));
    }

    Ok(Redirect::to("/settings"))
}

/// Catch-all route handler for any undefined routes
/// This will redirect any unknown path to the homepage
#[get("/<_..>", rank = 100)]
pub fn catch_all_redirect() -> Redirect {
    Redirect::to("/")
}
