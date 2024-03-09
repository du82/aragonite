use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::ContentType;
use std::io::Cursor;

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
    
        if let Some(cookie) = request.cookies().get("theme") {
            let theme = cookie.value();
    
            if let Some(body) = response.body().and_then(|b| b.into_string()) {
                let modified_body = body.replace("{{theme}}", &format!("themes/{}", theme));
                response.set_sized_body(Cursor::new(modified_body));
            }
        }
    }
}