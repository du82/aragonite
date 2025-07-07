// Theme and template middleware

use rand::seq::SliceRandom;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::ContentType;
use rocket::{Request, Response};
use std::fs;
use std::io::Cursor;
use std::process::Command;
use std::sync::Mutex;
use toml::Value;
use crate::filter::FilterManager;

pub struct ThemeMiddleware;

#[rocket::async_trait]
impl Fairing for ThemeMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "ThemeMiddleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let config_str = match fs::read_to_string("Config.toml") {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Unable to read Config.toml");
                return;
            }
        };

        let config: Value = match config_str.parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Unable to parse Config.toml");
                return;
            }
        };

        // Safe Search
        let mut safe_search_enabled = true;
        let mut safe_search_level = "moderate";
        let mut safe_search_value = "1";

        if let Some(filter_manager) = request.rocket().state::<Mutex<FilterManager>>() {
            let mut filter = filter_manager.lock().unwrap();

            if let Some(cookie) = request.cookies().get("safe_search_enabled") {
                if let Ok(enabled) = cookie.value().parse::<bool>() {
                    safe_search_enabled = enabled;
                    filter.enable(enabled);
                }
            }

            if let Some(cookie) = request.cookies().get("safe_search_level") {
                let level = cookie.value();
                if level == "off" || level == "moderate" || level == "strict" {
                    safe_search_level = level;
                    filter.set_level(level);

                    safe_search_value = match level {
                        "off" => "0",
                        "moderate" => "1",
                        "strict" => "2",
                        _ => "1",
                    };
                }
            }
        }

        if response.content_type() != Some(ContentType::HTML) {
            return;
        }

        // Theme
        let theme = request
            .cookies()
            .get("theme")
            .map(|cookie| cookie.value().to_string())
            .unwrap_or_else(|| "gemstone.css".to_string());

        if let Ok(body) = response.body_mut().to_string().await {
            let mut modified_body = body.replace("{{theme}}", &format!("themes/{}", theme));

            modified_body = modified_body.replace("{{safe_search_value}}", safe_search_value);

            if safe_search_enabled {
                modified_body = modified_body.replace("{{safe_search_checked}}", "checked");
                modified_body = modified_body.replace("{{safe_search_off_checked}}", "");
            } else {
                modified_body = modified_body.replace("{{safe_search_checked}}", "");
                modified_body = modified_body.replace("{{safe_search_off_checked}}", "checked");
            }

            modified_body = modified_body.replace("{{level_off_selected}}",
                if safe_search_level == "off" { "selected" } else { "" });
            modified_body = modified_body.replace("{{level_moderate_selected}}",
                if safe_search_level == "moderate" { "selected" } else { "" });
            modified_body = modified_body.replace("{{level_strict_selected}}",
                if safe_search_level == "strict" { "selected" } else { "" });

            let size = modified_body.len();
            response.set_sized_body(size, Cursor::new(modified_body));
        }

        if let Some(branding) = config.get("branding") {
            if let Some(name) = branding.get("name").and_then(Value::as_str) {
                if let Ok(body) = response.body_mut().to_string().await {
                    let modified_body = body.replace("{{name}}", name);
                    let size = modified_body.len();
                    response.set_sized_body(size, Cursor::new(modified_body));
                }
            }
        }

        if let Some(branding) = config.get("branding") {
            if let Some(greek) = branding.get("greek").and_then(Value::as_str) {
                if let Ok(body) = response.body_mut().to_string().await {
                    let modified_body = body.replace("{{greek}}", greek);
                    let size = modified_body.len();
                    response.set_sized_body(size, Cursor::new(modified_body));
                }
            }
        }

        if let Some(branding) = config.get("branding") {
            if let Some(mirror_updated) = branding.get("mirror_updated").and_then(Value::as_str) {
                if let Ok(body) = response.body_mut().to_string().await {
                    let modified_body = body.replace("{{mirror_updated}}", mirror_updated);
                    let size = modified_body.len();
                    response.set_sized_body(size, Cursor::new(modified_body));
                }
            }
        }

        // Search Bar Splash Text
        if let Some(features) = config.get("features") {
            if let Some(search_string_enabled) = features
                .get("search_string_enabled")
                .and_then(Value::as_bool)
            {
                if search_string_enabled {
                    if let Ok(body) = response.body_mut().to_string().await {
                        let file = match fs::read_to_string("search_string.txt") {
                            Ok(content) => content,
                            Err(_) => {
                                eprintln!("Unable to read search_string.txt");
                                return;
                            }
                        };

                        let lines: Vec<&str> = file.lines().collect();
                        let random_line = lines.choose(&mut rand::thread_rng()).unwrap_or(&"");

                        let modified_body = body.replace("{{search_string}}", random_line);
                        let size = modified_body.len();
                        response.set_sized_body(size, Cursor::new(modified_body));
                    }
                } else {
                    if let Ok(body) = response.body_mut().to_string().await {
                        let modified_body =
                            body.replace("{{search_string}}", "Search for something...");
                        let size = modified_body.len();
                        response.set_sized_body(size, Cursor::new(modified_body));
                    }
                }
            }
        }

        // Neofetch
        if let Some(features) = config.get("features") {
            if let Some(neofetch_enabled) =
                features.get("neofetch_enabled").and_then(Value::as_bool)
            {
                if neofetch_enabled {
                    if let Ok(body) = response.body_mut().to_string().await {
                        let neofetch_output = Command::new("neofetch").arg("--stdout").output();

                        match neofetch_output {
                            Ok(output) => {
                                let formatted_output = String::from_utf8_lossy(&output.stdout)
                                    .lines()
                                    .skip(2)
                                    .collect::<Vec<&str>>()
                                    .join("<br>");

                                let modified_body = body.replace("{{neofetch}}", &formatted_output);
                                let size = modified_body.len();
                                response.set_sized_body(size, Cursor::new(modified_body));
                            }
                            Err(_) => {
                                let modified_body = body.replace(
                                    "{{neofetch}}",
                                    "Neofetch is not installed on the host",
                                );
                                let size = modified_body.len();
                                response.set_sized_body(size, Cursor::new(modified_body));
                            }
                        }
                    }
                }
            }
        }
    }
}
