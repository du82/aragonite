use rocket::{get, post, routes, State};
use rocket::request::{Form, FromForm};
use rocket::response::content::Html;
use rocket::response::status;
use rocket::http::Status;
use toml::Value;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(FromForm)]
pub struct SearchForm {
    query: String,
}

#[derive(FromForm)]
struct SearchData {
    query: String,
}

#[get("/search?<results>")]
pub fn search_get(results: String) -> Html<String> {
    let search_results = perform_search(&results);
    let html = format!("<html><body><h1>Search Results:</h1><br><p>{}</p></body></html>", search_results);
    Html(html)
}

#[post("/search", data = "<search_data>")]
pub fn search_post(search_data: Form<SearchData>) -> Html<String> {
    let search_results = perform_search(&search_data.query);
    let search_results = search_results.replace("\n", "<br>");
    let mut file = File::open("static/internal.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = format!("{}<h1>Search Results for: {}</h1><p>{}</p>", contents, search_data.query,search_results);
    Html(contents)
}

pub fn perform_search(query: &str) -> String {
    // Read the TOML file
    let data = fs::read_to_string("Data.toml").unwrap();
    let toml: Value = toml::from_str(&data).unwrap();

    // Perform the search in the TOML file
    let mut results = String::new();
    let query = query.to_lowercase(); // Convert the query to lowercase
    for (book, info) in toml.as_table().unwrap().iter() {
        if let Some(title) = info.get("title") {
            let title = title.as_str().unwrap().to_lowercase(); // Convert the title to lowercase
            if title.contains(&query) {
                results.push_str(&format!("{}: {}\n", book, info));
            }
        }
    }
    results
}