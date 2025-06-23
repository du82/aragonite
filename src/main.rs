// Application entry point

#[macro_use]
extern crate rocket;

use rocket::{routes, Build, Rocket};
use std::collections::HashMap;
use std::sync::Mutex;
pub mod benchmarks;
pub mod common;
pub mod filter;
mod middleware;
mod routes;
mod search;
use middleware::ThemeMiddleware;

#[launch]
fn rocket() -> Rocket<Build> {
    // Initialize filter manager
    let filter_manager = filter::FilterManager::new();

    rocket::build()
        .mount(
            "/",
            routes![
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
                routes::safe_search_toggle,
                routes::safe_search_level,
                search::search_get,
                search::search_post,
                search::random_search,
            ],
        )
        .attach(ThemeMiddleware)
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .manage(Mutex::new(filter_manager))
}
