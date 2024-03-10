#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use toml::Value;

mod auth;
mod search;
mod middleware;
use middleware::ThemeMiddleware;

fn main() {

    rocket::ignite()
        .mount("/", routes![
            auth::index,
            auth::images, 
            auth::videos, 
            auth::news, 
            auth::porn, 
            auth::obscure, 
            auth::dw, 
            auth::theme_selector, 
            auth::light_theme, 
            auth::dark_theme, 
            auth::high_contrast_theme,
            auth::midnight_theme,
            auth::clown_theme,
            auth::weed_theme, 
            search::search_get, 
            search::search_post, 
            search::news_get,
            search::news_post
            ]
        )
        .attach(ThemeMiddleware)
        .manage(Mutex::new(HashMap::<String, String>::new())) // This is the session store
        .launch();
}