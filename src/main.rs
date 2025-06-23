// Application entry point

#[macro_use]
extern crate rocket;

use rocket::{routes, Build, Rocket};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;
pub mod benchmarks;
pub mod common;
pub mod filter;
mod middleware;
mod routes;
mod search;
use middleware::ThemeMiddleware;

// Remove all files in a directory
fn clear_directory(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        return Ok(());
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            fs::remove_dir_all(entry_path)?;
        } else {
            fs::remove_file(entry_path)?;
        }
    }
    
    Ok(())
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Check for reindex flag
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args.contains(&"-i".to_string()) {
        println!("Reindex flag detected. Clearing index directory...");
        let index_path = Path::new("index");
        if let Err(err) = clear_directory(index_path) {
            eprintln!("Error clearing index directory: {}", err);
        } else {
            println!("Index directory cleared successfully.");
        }
    }
    
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
