use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::ContentType;
use rand::seq::SliceRandom;
use std::io::Cursor;
use std::fs;
pub struct ThemeMiddleware;
impl Fairing for ThemeMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "ThemeMiddleware",
            kind: Kind::Response
        }
    }
    fn on_response(&self, request: &Request, response: &mut Response) {
        // Only modify HTML responses
        if response.content_type() != Some(ContentType::HTML) {
            return;
        }

        let theme = request.cookies().get("theme").map(|cookie| cookie.value().to_string()).unwrap_or_else(|| "gemstone.css".to_string());

        if let Some(body) = response.body().and_then(|b| b.into_string()) {
            let modified_body = body.replace("{{theme}}", &format!("themes/{}", theme));
            response.set_sized_body(Cursor::new(modified_body));
        }

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
    }
}