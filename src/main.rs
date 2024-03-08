#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use toml::Value;

mod auth;
mod search;

fn load_users() -> Result<HashMap<String, (String, String)>, Box<dyn std::error::Error>> {
    let mut users_map = HashMap::new();

    let data = fs::read_to_string("Users.toml")?;
    let value: Value = toml::from_str(&data)?;

    if let Value::Table(tables) = value {
        for (user_id, table) in tables {
            if let Value::Table(user_info) = table {
                if let Some(Value::String(password)) = user_info.get("password") {
                    let display_name = user_info.get("display_name")
                        .and_then(|v| v.as_str())
                        .or_else(|| user_info.get("name").and_then(|v| v.as_str()))
                        .unwrap_or("");
                    users_map.insert(user_id, (display_name.to_string(), password.to_string()));
                }
            }
        }
    }

    Ok(users_map)
}

fn main() {
    let users_map = load_users().expect("Unable to load users");

    rocket::ignite()
        .mount("/", routes![auth::index, auth::login, auth::login_post, auth::logout, auth::internal, auth::images, auth::videos, auth::news, auth::porn, auth::dw, auth::css_styling, search::search_get, search::search_post])
        .manage(auth::User { logged_in: Mutex::new(false) })
        .manage(users_map)
        .manage(Mutex::new(HashMap::<String, String>::new())) // This is the session store
        .launch();
}