#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, post, routes, State};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;

mod auth;

fn main() {
    let users_data = fs::read_to_string("users.txt").expect("Unable to read file");
    let mut users_map: HashMap<String, String> = HashMap::new();

    for line in users_data.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            users_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    rocket::ignite()
        .mount("/", routes![auth::index, auth::login, auth::login_post, auth::logout, auth::internal])
        .manage(auth::User { logged_in: Mutex::new(false) })
        .manage(users_map)
        .launch();
}