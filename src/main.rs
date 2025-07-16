// Application entry point

extern crate rocket;

use rocket::{routes, Build, Rocket};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::net::TcpListener;
use std::path::Path;
use std::sync::Mutex;
pub mod benchmarks;
pub mod common;
pub mod filter;
mod middleware;
mod routes;
mod search;
use middleware::ThemeMiddleware;

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

fn rocket() -> Rocket<Build> {
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
                routes::catch_all_redirect,
            ],
        )
        .attach(ThemeMiddleware)
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .manage(Mutex::new(filter_manager))
}

fn find_available_port(start_port: u16, max_attempts: u16) -> Option<u16> {
    for port in start_port..(start_port + max_attempts) {
        // Try binding to both addresses to ensure the port is truly available
        if TcpListener::bind(("127.0.0.1", port)).is_ok() && 
           TcpListener::bind(("0.0.0.0", port)).is_ok() {
            return Some(port);
        }
    }
    None
}

#[rocket::main]
async fn main() {
    // Try to get configuration from Rocket.toml
    let config = rocket::Config::figment().extract::<rocket::Config>();
    
    // Default to port 8000 if config can't be loaded
    let default_port = config.map(|c| c.port).unwrap_or(8000);
    let max_attempts = 100; // Try up to 100 ports
    
    // Find an available port
    let port = match find_available_port(default_port, max_attempts) {
        Some(p) => p,
        None => {
            eprintln!("Could not find an available port after {} attempts, starting from port {}", max_attempts, default_port);
            std::process::exit(1);
        }
    };
    
    if port != default_port {
        println!("Default port {} is in use, using port {} instead", default_port, port);
    }
    
    // Configure Rocket with the available port
    let figment = rocket::Config::figment()
        .merge(("port", port));
    
    // Launch the application
    if let Err(e) = rocket().configure(figment).launch().await {
        eprintln!("Failed to launch: {}", e);
        std::process::exit(1);
    }
}