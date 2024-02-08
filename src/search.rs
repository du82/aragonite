use rocket::{get, post, routes, State};
use rocket::request::{Form, FromForm};
use rocket::response::content::Html;
use toml::Value;
use std::fs;
use std::fs::File;
use std::io::Read;
use toml::value::Table;
use rocket::http::Cookies;
use std::collections::HashSet;

#[derive(FromForm)]
struct SearchData {
    query: String,
}

#[post("/search", data = "<search_data>")]
pub fn search_post(mut cookies: Cookies, search_data: Form<SearchData>) -> Html<String> {
    let trimmed_query = search_data.query.trim_start();
    if trimmed_query.len() < 2 || search_data.query.len() > 50 {
        return Html("<h1>Error: Search query must be between 2 and 50 characters long.</h1>".to_string());
    }
    let start_time = std::time::Instant::now();
    let search_results = perform_search(&search_data.query);
    let elapsed_time = start_time.elapsed();
    let num_results = search_results.len();
    let result_word = if num_results == 1 { "result" } else { "results" };
    let search_results = search_results.iter().map(|s| format!("<div class='search-result'>{}</div>", s.replace("\n", "<br>"))).collect::<Vec<_>>();
    let search_results = search_results.join("");
    let search_results = format!("<div class='results-container'>{}</div>", search_results);
    let search_results_footer  = format!("<div class='footer-container'><div class='footer-box'><p>Footer</p></div></div>");
    let mut file = File::open("static/internal.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let display_name = cookies.get_private("display_name").map(|cookie| cookie.value().to_string()).unwrap_or_else(|| "Guest".to_string());
    contents = contents.replace("{{display_name}}", &display_name);
    let statistics = format!("<div><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, search_data.query, elapsed_time.as_millis());
    contents = contents.replace("{{statistics}}", &statistics);
    contents = format!("{}{}{}", contents, search_results, search_results_footer);
    Html(contents)
}

#[get("/search?<results>")]
pub fn search_get(results: String) -> Html<String> {
    let trimmed_results = results.trim_start();
    if trimmed_results.len() < 2 || results.len() > 50 {
        return Html("<div id='search-results'>Error: Search query must be between 2 and 50 characters long.</div>".to_string());
    }
    let search_results = perform_search(&results);
    let num_results = search_results.len();
    let result_word = if num_results == 1 { "result" } else { "results" };
    let search_results = search_results.iter().map(|s| format!("<div class='search-result'>{}</div>", s.replace("\n", "<br>"))).collect::<Vec<_>>();
    let search_results = search_results.join("");
    let search_results = format!("<div class='results-container'>{}</div>", search_results);
    let html = format!("<html><body><h1>{} {}:</h1>{}<br></body></html>", num_results, result_word, search_results);
    Html(html)
}

fn format_toml_table(table: &Table) -> String {
    let title = table.get("title").unwrap().as_str().unwrap();
    let other_fields = table.iter()
        .filter(|&(k, _)| k != "title")
        .map(|(k, v)| {
            let key = {
                let k = k.replace("_", " ");
                let mut chars = k.chars();
                let first_char = chars.next().unwrap().to_uppercase();
                let rest = chars.collect::<String>();
                format!("{}{}", first_char, rest)
            };
            let value = match v {
                Value::String(s) => {
                    // Check if the string is a URL
                    if s.starts_with("http://") || s.starts_with("https://") {
                        // If it is, format it as an HTML link
                        format!("<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>", s, s)
                    } else {
                        s.clone()
                    }
                },
                _ => v.to_string(),
            };
            format!("{}: {}", key, value)
        })
        .collect::<Vec<_>>();
    format!("<h3>{}</h3>\n{}", title, other_fields.join("\n"))
}

pub fn perform_search(query: &str) -> HashSet<String> {
    // Read the TOML file
    let data = fs::read_to_string("Data.toml").unwrap();
    let toml: Value = toml::from_str(&data).unwrap();

    // Perform the search in the TOML file
    let mut results = HashSet::new();
    let query = query.to_lowercase(); // Convert the query to lowercase
    for (search_result, info) in toml.as_table().unwrap().iter() {
        if let Some(title) = info.get("title") {
            let title = title.as_str().unwrap().to_lowercase(); // Convert the title to lowercase
            if title.contains(&query) {
                let formatted_info = format_toml_table(info.as_table().unwrap());
                results.insert(format!("{}\n", formatted_info));
                continue;
            }
        }
        if let Some(description) = info.get("description") {
            let description = description.as_str().unwrap().to_lowercase(); // Convert the description to lowercase
            if description.contains(&query) {
                let formatted_info = format_toml_table(info.as_table().unwrap());
                results.insert(format!("{}\n", formatted_info));
            }
        }
        if let Some(author) = info.get("author") {
            let author = author.as_str().unwrap().to_lowercase(); // Convert the description to lowercase
            if author.contains(&query) {
                let formatted_info = format_toml_table(info.as_table().unwrap());
                results.insert(format!("{}\n", formatted_info));
            }
        }
    }

    results
}