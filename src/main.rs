#[macro_use] extern crate rocket;

use rocket::{routes, Build, Rocket};
use std::sync::Mutex;
use std::collections::HashMap;
pub mod benchmarks;
pub mod common;
mod routes;
mod search;
mod middleware;
use middleware::ThemeMiddleware;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![
            routes::logo_image,
            routes::index,
            routes::favicon,
            routes::donate,
            routes::about,
            routes::privacy,
            routes::tos,
            routes::mirrors,
            routes::settings,
            routes::syntax,
            routes::theme_selector,
            routes::gemstone_theme,
            routes::gemstone_light_theme,
            routes::high_contrast_theme,
            routes::midnight_theme,
            search::search_get,
            search::search_post,
            search::mobile_search_get,
            search::mobile_search_post,
            search::random_search,
        ])
        .attach(ThemeMiddleware)
        .manage(Mutex::new(HashMap::<String, String>::new())) // This is the session store
}
