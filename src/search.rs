use rocket::{get, post, routes, State};
use rocket::request::{Form, FromForm, Query};
use rocket::response::content::Html;
use toml::Value;
use std::fs;
use std::fs::File;
use std::io::Read;
use toml::value::Table;
use rocket::http::Cookies;
use std::collections::HashSet;
use rocket::response::Redirect;
use rocket::uri;
use crate::auth::current_theme;

#[derive(FromForm)]
struct SearchData {
    query: String,
}
#[derive(FromForm)]
struct SearchForm {
    query: String,
}

#[post("/search", data = "<search_form>")]
pub fn search_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(search_get: q = search_form.into_inner().query))
}

#[get("/search?<q>")]
pub fn search_get(q: Option<String>, cookies: Cookies) -> Result<Html<String>, Redirect> {
    match q {
        Some(query) => {
            let trimmed_query = query.trim_start();
            if trimmed_query.len() < 2 || query.len() > 50 {
                return Ok(Html("<h1>Error: Search query must be between 2 and 50 characters long.</h1>".to_string()));
            }
            let start_time = std::time::Instant::now();
            let search_results = perform_search(&query);
            let elapsed_time = start_time.elapsed();
            let num_results = search_results.len();
            let result_word = if num_results == 1 { "results" } else { "results" };
            let search_results = search_results.iter().map(|s| format!("<div class='search-result'>{}</div>", s.replace("\n", "<br>"))).collect::<Vec<_>>();
            let search_results = search_results.join("");
            let search_results = format!("<div class='results-container'>{}</div>", search_results);
            let search_results_footer  = format!("<div class='footer-container'><div class='footer-box'><p>Footer</p></div></div>");
            let mut file = File::open("static/serps/web.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let statistics = format!("<div style='position: fixed; right: 10px; margin: 0; padding: 0;'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, elapsed_time.as_millis());
            let theme = current_theme(&cookies);
            contents = contents.replace("{{theme}}", &theme);
            contents = contents.replace("{{statistics}}", &statistics);
            contents = format!("{}{}{}", contents, search_results, search_results_footer);
            Ok(Html(contents))
        },
        None => Err(Redirect::to("/")),
    }
}

#[post("/news", data = "<search_form>")]
pub fn news_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(news_get: q = search_form.into_inner().query))
}

#[get("/news?<q>")]
pub fn news_get(q: Option<String>) -> Result<Html<String>, Redirect> {
    match q {
        Some(query) => {
            let trimmed_query = query.trim_start();
            if trimmed_query.len() < 2 || query.len() > 50 {
                return Ok(Html("<h1>Error: Search query must be between 2 and 50 characters long.</h1>".to_string()));
            }
            let start_time = std::time::Instant::now();
            let search_results = perform_search(&query);
            let elapsed_time = start_time.elapsed();
            let num_results = search_results.len();
            let result_word = if num_results == 1 { "article" } else { "articles" };
            let search_results = search_results.iter().map(|s| format!("<div class='search-result'>{}</div>", s.replace("\n", "<br>"))).collect::<Vec<_>>();
            let search_results = search_results.join("");
            let search_results = format!("<div class='results-container'>{}</div>", search_results);
            let search_results_footer  = format!("<div class='footer-container'><div class='footer-box'><p>Footer</p></div></div>");
            let mut file = File::open("static/serps/news.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let statistics = format!("<div style='position: fixed; right: 10px; margin: 0; padding: 0;'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, elapsed_time.as_millis());
            contents = contents.replace("{{statistics}}", &statistics);
            contents = format!("{}{}{}", contents, search_results, search_results_footer);
            Ok(Html(contents))
        },
        None => {
            // Return the news page here
            let mut file = File::open("static/serps/news.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(Html(contents))
        },
    }
}

#[post("/obscure", data = "<search_form>")]
pub fn obscure_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(obscure_get: q = search_form.into_inner().query))
}

#[get("/obscure?<q>")]
pub fn obscure_get(q: Option<String>) -> Result<Html<String>, Redirect> {
    match q {
        Some(query) => {
            let trimmed_query = query.trim_start();
            if trimmed_query.len() < 2 || query.len() > 50 {
                return Ok(Html("<h1>Error: Search query must be between 2 and 50 characters long.</h1>".to_string()));
            }
            let start_time = std::time::Instant::now();
            let search_results = perform_search(&query);
            let elapsed_time = start_time.elapsed();
            let num_results = search_results.len();
            let result_word = if num_results == 1 { "obscurity" } else { "obscurities" };
            let search_results = search_results.iter().map(|s| format!("<div class='search-result'>{}</div>", s.replace("\n", "<br>"))).collect::<Vec<_>>();
            let search_results = search_results.join("");
            let search_results = format!("<div class='results-container'>{}</div>", search_results);
            let search_results_footer  = format!("<div class='footer-container'><div class='footer-box'><p>Footer</p></div></div>");
            let mut file = File::open("static/serps/obscure.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let statistics = format!("<div style='position: fixed; right: 10px; margin: 0; padding: 0;'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, elapsed_time.as_millis());
            contents = contents.replace("{{statistics}}", &statistics);
            contents = format!("{}{}{}", contents, search_results, search_results_footer);
            Ok(Html(contents))
        },
        None => {
            // Return the news page here
            let mut file = File::open("static/serps/obscure.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(Html(contents))
        },
    }
}

fn format_toml_table(table: &Table) -> String {
    let title = table.get("title").unwrap().as_str().unwrap();
    let description = table.get("description").unwrap().as_str().unwrap();
    let description = if description.len() > 200 {
        format!("{}...", description.chars().take(200).collect::<String>())
    } else {
        description.to_string()
    };
    let mut url_fields = Vec::new();
    let mut other_fields = Vec::new();

    for (k, v) in table.iter() {
        if k == "title" {
            continue;
        }

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
                    let url = format!("<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>", s, s);
                    url_fields.push(format!("{}: {}", key, url));
                    continue;
                } else {
                    s.clone()
                }
            },
            _ => v.to_string(),
        };
        other_fields.push(format!("{}: {}", key, value));
    }

    let url_fields = url_fields.join("\n");
    let other_fields = other_fields.join("\n");

    format!("{}<h3>{}</h3>\n{}<details class='more-info'><summary>More Info (metadata)</summary><div>{}</div></details>", url_fields, title, description, other_fields)
}

pub fn perform_search(query: &str) -> HashSet<String> {
    // Read the TOML file
    let data = fs::read_to_string("Data.toml").unwrap();
    let toml: Value = toml::from_str(&data).unwrap();

    // Perform the search in the TOML file
    let mut results = HashSet::new();
    let query = query.to_lowercase(); // Convert the query to lowercase

    if query.starts_with("author:") {
        let author_query = &query[7..]; // Remove "author=" from the query
        for (search_result, info) in toml.as_table().unwrap().iter() {
            if let Some(author) = info.get("author") {
                let author = author.as_str().unwrap().to_lowercase(); // Convert the author to lowercase
                if author == author_query {
                    let formatted_info = format_toml_table(info.as_table().unwrap());
                    results.insert(format!("{}\n", formatted_info));
                }
            }
        }
    } if query.starts_with("site:") {
        let clearnet_query = &query[5..]; // Remove "clearnet=" from the query
        for (search_result, info) in toml.as_table().unwrap().iter() {
            if let Some(clearnet) = info.get("clearnet") {
                let clearnet = clearnet.as_str().unwrap().to_lowercase(); // Convert the clearnet URL to lowercase
                if clearnet.contains(clearnet_query) {
                    let formatted_info = format_toml_table(info.as_table().unwrap());
                    results.insert(format!("{}\n", formatted_info));
                }
            }
        }

    } if query.starts_with("year=") {
        let year_query = &query[4..]; // Remove "clearnet=" from the query
        for (search_result, info) in toml.as_table().unwrap().iter() {
            if let Some(year_published) = info.get("year_published") {
                let year_published = year_published.as_str().unwrap().to_lowercase(); // Convert the clearnet URL to lowercase
                if year_published.contains(year_query) {
                    let formatted_info = format_toml_table(info.as_table().unwrap());
                    results.insert(format!("{}\n", formatted_info));
                }
            }
        }

    } if query.starts_with("category:") {
        let category_query = &query[9..]; // Remove "clearnet=" from the query
        for (search_result, info) in toml.as_table().unwrap().iter() {
            if let Some(category) = info.get("category") {
                let category = category.as_str().unwrap().to_lowercase(); // Convert the clearnet URL to lowercase
                if category.contains(category_query) {
                    let formatted_info = format_toml_table(info.as_table().unwrap());
                    results.insert(format!("{}\n", formatted_info));
                }
            }
        }
        
    } else {
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
        }
    }

    results
}