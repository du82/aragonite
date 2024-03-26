#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;
use std::sync::Mutex;
use std::collections::HashMap;

mod auth;
mod search;
mod middleware;
use middleware::ThemeMiddleware;


fn main() {
    rocket::ignite()
        .mount("/", routes![
            auth::logo_image, 
            auth::index,
            auth::favicon,
            auth::donate,
            auth::about,
            auth::privacy,
            auth::tos,
            auth::jobs,
            auth::settings,
            auth::theme_selector,
            auth::gemstone_theme,
            auth::gemstone_light_theme,
            auth::high_contrast_theme,
            auth::midnight_theme,
            search::search_get, 
            search::search_post,
            search::mobile_search_get,
            search::mobile_search_post,
            search::random_search,
            ]
        )
        .attach(ThemeMiddleware)
        .manage(Mutex::new(HashMap::<String, String>::new())) // This is the session store
        .launch();
}