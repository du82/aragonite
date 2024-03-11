use crate::auth::current_theme;
use rocket::http::Cookies;
use rocket::request::{Form, FromForm, Query};
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::uri;
use rocket::{get, post, routes, State};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Read;
use toml::value::Table;
use toml::Value;

// Experimental
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, FAST, STORED, TEXT};
use tantivy::Document;
use tantivy::Index;
use tantivy::ReloadPolicy;

#[derive(FromForm)]
struct SearchData {
    query: String,
}
#[derive(FromForm)]
struct SearchForm {
    query: String,
}

pub struct Searcher {
    pub results: Vec<String>,
}

#[post("/search", data = "<search_form>")]
pub fn search_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(search_get: q = search_form.into_inner().query))
}

#[get("/search?<q>")]
pub fn search_get(q: Option<String>, cookies: Cookies) -> Result<Html<String>, Redirect> {
    match tantivy_processor(q.as_deref().unwrap_or("")) {
        Ok(results) => {
            println!("{:#?}", results);
            match q {
                Some(query) => {
                    let trimmed_query = query.trim_start();
                    if trimmed_query.len() < 2 || query.len() > 50 {
                        return Ok(Html(
                            "<h1>Error: Search query must be between 2 and 50 characters long.</h1>"
                                .to_string(),
                        ));
                    }
                    let start_time = std::time::Instant::now();
                    let elapsed_time = start_time.elapsed();
                    let num_results = results.len();
                    let result_word = if num_results == 1 {
                        "results"
                    } else {
                        "results"
                    };
                    let mut search_results_new = format!("<div class='results-container'>{:?}</div>", results);
                    //let search_results_footer = format!(
                    //    "<div class='footer-container'><div class='footer-box'><p>{:?}</p></div></div>", results
                    //);
                    let mut file = File::open("static/serps/web.html").unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let statistics = format!("<div style='position: fixed; right: 10px; margin: 0; padding: 0;'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, elapsed_time.as_millis());
                    let theme = current_theme(&cookies);
                    contents = contents.replace("{{statistics}}", &statistics);
                    search_results_new = search_results_new.replace("test", "");
                    contents = format!("{}{}", contents, search_results_new);
                    Ok(Html(contents))
                },
                None => Err(Redirect::to("/"))
            }
        },
        Err(e) => {
            println!("Error creating index: {:?}", e);
            Err(Redirect::to("/"))
        }
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
                return Ok(Html(
                    "<h1>Error: Search query must be between 2 and 50 characters long.</h1>"
                        .to_string(),
                ));
            }
            let start_time = std::time::Instant::now();
            let elapsed_time = start_time.elapsed();
            let search_results_footer = format!(
                "<div class='footer-container'><div class='footer-box'><p>Footer</p></div></div>"
            );
            let mut file = File::open("static/serps/news.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents = format!("{}{}", contents, search_results_footer);
            Ok(Html(contents))
        }
        None => {
            // Return the news page here
            let mut file = File::open("static/serps/news.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(Html(contents))
        }
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
                return Ok(Html(
                    "<h1>Error: Search query must be between 2 and 50 characters long.</h1>"
                        .to_string(),
                ));
            }
            let start_time = std::time::Instant::now();
            let elapsed_time = start_time.elapsed();
            let search_results_footer = format!(
                "<div class='footer-container'><div class='footer-box'><p>Footer</p></div></div>"
            );
            let mut file = File::open("static/serps/obscure.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents = format!("{}{}", contents, search_results_footer);
            Ok(Html(contents))
        }
        None => {
            // Return the news page here
            let mut file = File::open("static/serps/obscure.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(Html(contents))
        }
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
                    let url = format!(
                        "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                        s, s
                    );
                    url_fields.push(format!("{}: {}", key, url));
                    continue;
                } else {
                    s.clone()
                }
            }
            _ => v.to_string(),
        };
        other_fields.push(format!("{}: {}", key, value));
    }

    let url_fields = url_fields.join("");
    let other_fields = other_fields.join("");

    format!("{}<h3>{}</h3>\n{}<details class='more-info'><summary>More Info (metadata)</summary><div>{}</div></details>", url_fields, title, description, other_fields)
}


pub fn tantivy_processor(query: &str) -> tantivy::Result<Vec<String>> {

    let index_dir = "index";
    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let description = schema_builder.add_text_field("description", TEXT | STORED);
    let year_published = schema_builder.add_i64_field("year_published", STORED | FAST);
    let year_updated = schema_builder.add_i64_field("year_updated", STORED | FAST);
    let url = schema_builder.add_text_field("clearnet", TEXT | STORED);
    let onion = schema_builder.add_text_field("onion", TEXT | STORED);
    let status = schema_builder.add_text_field("status", TEXT | STORED);
    let category = schema_builder.add_text_field("category", TEXT | STORED);
    let tags = schema_builder.add_text_field("tags", TEXT | STORED | FAST);
    let schema = schema_builder.build();
    let index = if fs::read_dir(index_dir)?.count() == 0 {
        // Define the schema for the index
    let mut schema_builder = Schema::builder();

    // Create the index
    let index = Index::create_in_dir("index", schema.clone())?;

    // Parse the TOML file
    let data = fs::read_to_string("Data.toml")
        .map_err(|e| tantivy::TantivyError::InvalidArgument(format!("{}", e)))?;
    let value: Value = toml::from_str(&data)
        .map_err(|e| tantivy::TantivyError::InvalidArgument(format!("{}", e)))?;

    // Add data to the index
    let mut index_writer = index.writer(50_000_000)?;
    if let Some(tables) = value.as_table() {
        for (_name, table) in tables {
            let mut doc = Document::new();
            for (key, value) in table.as_table().unwrap() {
                match schema.get_field(key) {
                    Ok(field) => match value {
                        Value::String(s) => doc.add_text(field, s),
                        Value::Integer(i) => doc.add_i64(field, *i),
                        Value::Array(a) => {
                            for v in a {
                                if let Value::String(s) = v {
                                    doc.add_text(field, s);
                                }
                            }
                        }
                        _ => continue,
                    },
                    Err(_) => continue,
                }
            }
            index_writer.add_document(doc);
        }
    }
    index_writer.commit()?;
    index
    } else {
        Index::open_in_dir(index_dir)?
    };

    // Searching the index
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    // Querying the index
    let query_parser = QueryParser::for_index(&index, vec![title, description, category, tags]);
    let query = query_parser.parse_query(query)?;

    // Collectors
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let html = document_to_html(&retrieved_doc, &schema);
        let html_without_quotes = html.replace(", ", "");
        results.push(html_without_quotes)
    }
    for result in &results {
        println!("{}", result);
    }

    tantivy_renderer(&results);

    let results_str = results.join("\n");
    println!("{}", results_str);
    Ok(results)
}

pub fn tantivy_renderer(results: &Vec<String>) {
    for result in results {
        let result_without_json_formatting = result.replace("{", "")
            .replace("}", "")
            .replace(":", "")
            .replace("\"", "")
            .replace(", ", "");
        println!("{}", result_without_json_formatting);
    }
}

use std::collections::HashMap;

fn document_to_html(doc: &Document, schema: &Schema) -> String {
    let mut fields = HashMap::new();
    for field_value in doc.field_values() {
        let field_name = schema.get_field_name(field_value.field());
        if let tantivy::schema::Value::Str(value) = field_value.value() {
            let value_without_newlines = value.replace("\n", "").replace("[", "");
            fields.insert(field_name, value_without_newlines);
        }
    }

    let default_value = "".to_string();
    let title = fields.get("title").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");
    let description = fields.get("description").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");
    let url = fields.get("clearnet").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");
    let tags = fields.get("tags").unwrap_or_else(|| &&default_value).replace(", ", "");
    let category = fields.get("category").unwrap_or_else(|| &&default_value).replace(", ", "");

    format!("<div class='search-result'><h3><a href='{}'>{}</a></h3><p>{}</p><details class='more-info'><summary>More Info (metadata)</summary><div><br>Tags: {}<br>Category: {}</div></details></div>", url, title, description, tags, category)
}