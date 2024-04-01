use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::ContentType;
use rand::seq::SliceRandom;
use std::io::Cursor;
use std::process::Command;
use std::fs;
use toml::Value;
pub struct ThemeMiddleware;
impl Fairing for ThemeMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "ThemeMiddleware",
            kind: Kind::Response
        }
    }
    fn on_response(&self, request: &Request, response: &mut Response) {

        let config_str = fs::read_to_string("Config.toml").unwrap();
        let config: Value = config_str.parse().unwrap();

        // Only modify HTML responses
        if response.content_type() != Some(ContentType::HTML) {
            return;
        }

        // Read the SVG file
        let svg_content = fs::read_to_string("radar.svg").unwrap_or_else(|_| String::new());

        // Replace {{radar_graph}} with the SVG content
        if let Some(body) = response.body().and_then(|b| b.into_string()) {
            let modified_body = body.replace("{{radar_graph}}", &svg_content);
            response.set_sized_body(Cursor::new(modified_body));
        }

        // Inject the theme if the cookie is different from default
        let theme = request.cookies().get("theme").map(|cookie| cookie.value().to_string()).unwrap_or_else(|| "gemstone.css".to_string());
        if let Some(body) = response.body().and_then(|b| b.into_string()) {
            let modified_body = body.replace("{{theme}}", &format!("themes/{}", theme));
            response.set_sized_body(Cursor::new(modified_body));
        }

        // Set name
        if let Some(branding) = config.get("branding") {
            if let Some(name) = branding.get("name").and_then(Value::as_str) {
                if let Some(body) = response.body().and_then(|b| b.into_string()){
                    // Replace {{search_string}} with the random line
                    let modified_body = body.replace("{{name}}", name);
                    response.set_sized_body(Cursor::new(modified_body));
                }
            }
        }

        // Set greek letter
        if let Some(branding) = config.get("branding") {
            if let Some(greek) = branding.get("greek").and_then(Value::as_str) {
                if let Some(body) = response.body().and_then(|b| b.into_string()){
                    // Replace {{search_string}} with the random line
                    let modified_body = body.replace("{{greek}}", greek);
                    response.set_sized_body(Cursor::new(modified_body));
                }
            }
        }

        // Set mirror updated date
        if let Some(branding) = config.get("branding") {
            if let Some(mirror_updated) = branding.get("mirror_updated").and_then(Value::as_str) {
                if let Some(body) = response.body().and_then(|b| b.into_string()){
                    // Replace {{search_string}} with the random line
                    let modified_body = body.replace("{{mirror_updated}}", mirror_updated);
                    response.set_sized_body(Cursor::new(modified_body));
                }
                
            }
        }

        if let Some(features) = config.get("features") {
            if let Some(search_string_enabled) = features.get("search_string_enabled").and_then(Value::as_bool) {
                if search_string_enabled {
                    if let Some(body) = response.body().and_then(|b| b.into_string()) {
                        // Read the file
                        let file = fs::read_to_string("search_string.txt").expect("Unable to read file");
                        let lines: Vec<&str> = file.lines().collect();
                    
                        // Select a random line
                        let random_line = lines.choose(&mut rand::thread_rng()).unwrap_or(&"");
                    
                        // Replace {{search_string}} with the random line
                        let modified_body = body.replace("{{search_string}}", random_line);
                        response.set_sized_body(Cursor::new(modified_body));
                    }
                } else {
                    if let Some(body) = response.body().and_then(|b| b.into_string()) {
                        // Replace {{search_string}} with "Search for something..."
                        let modified_body = body.replace("{{search_string}}", "Search for something...");
                        response.set_sized_body(Cursor::new(modified_body));
                    }
                }
            }
        }

        if let Some(features) = config.get("features") {
            if let Some(neofetch_enabled) = features.get("neofetch_enabled").and_then(Value::as_bool) {
                if neofetch_enabled {
                    if let Some(body) = response.body().and_then(|b| b.into_string()) {
                        // Try to execute neofetch command
                        let neofetch_output = Command::new("neofetch")
                            .arg("--stdout")
                            .output();
                    
                        match neofetch_output {
                            Ok(output) => {
                                // Format the output
                                let formatted_output = String::from_utf8_lossy(&output.stdout)
                                    .lines()
                                    .skip(2)
                                    .collect::<Vec<&str>>()
                                    .join("<br>");
                    
                                // Replaces {{neofetch}} with the command output
                                let modified_body = body.replace("{{neofetch}}", &formatted_output);
                                response.set_sized_body(Cursor::new(modified_body));
                            },
                            Err(_) => {
                                let modified_body = body.replace("{{neofetch}}", "Neofetch is not installed on the host");
                                response.set_sized_body(Cursor::new(modified_body));
                            }
                        }
                    }
                } else {
                    return;
                }
            }
        }
    }
}