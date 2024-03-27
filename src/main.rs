#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;
use std::sync::Mutex;
use std::collections::HashMap;

mod routes;
mod search;
mod middleware;
use middleware::ThemeMiddleware;


fn main() {
    rocket::ignite()
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
            ]
        )
        .attach(ThemeMiddleware)
        .manage(Mutex::new(HashMap::<String, String>::new())) // This is the session store
        .launch();
}