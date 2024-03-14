
use rocket::request::{Form, FromForm};
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::uri;
use rocket::{get, post};
use std::fs;
use std::fs::File;
use std::io::Read;
use toml::Value;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, FAST, STORED, TEXT};
use tantivy::Document;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tantivy::query::BooleanQuery;
use tantivy::query::Occur;
use tantivy::query::Query;
use tantivy::Term;
use tantivy::query::TermQuery;
use tantivy::schema::IndexRecordOption;
use std::time::{Duration, Instant};

#[derive(FromForm)]
struct SearchData {
    query: String,
}
#[derive(FromForm)]
struct SearchForm {
    query: String,
}

use std::time::SystemTime;

#[get("/random")]
pub fn random_search() -> Result<Redirect, std::io::Error> {
    let content = fs::read_to_string("random.txt")?;
    let queries: Vec<&str> = content.lines().collect();
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let query = queries[(time as usize) % queries.len()];
    Ok(Redirect::to(uri!(search_get: q = query.to_string())))
}

#[post("/search", data = "<search_form>")]
pub fn search_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(search_get: q = search_form.into_inner().query))
}

#[get("/search?<q>")]
pub fn search_get(q: Option<String>) -> Result<Html<String>, Redirect> {
    match tantivy_processor(q.as_deref().unwrap_or("")) {
        Ok((results, duration)) => {
            //println!("{:#?}", results);
            match q {
                Some(query) => {
                    let trimmed_query = query.trim_start();
                    if trimmed_query.len() < 1 || query.len() > 128 {
                        return Ok(Html(
                            "<h1>Error: Search query must be between 1 and 128 characters long.</h1>"
                                .to_string(),
                        ));
                    }
                    let num_results = results.len();
                    let result_word = if num_results == 1 {
                        "results"
                    } else {
                        "results"
                    };
                    let mut search_results = format!("<div class='results-container'>{:?}</div>", results);
                    let search_results_footer = format!(
                        "<footer>
                        <div class='footer-container'>
                            <a href='about'>
                            About Us
                            </a>|
                            <a href='privacy-policy'>
                            Privacy Policy
                            </a>|
                            <a href='careers'>
                            Careers
                            </a>|
                            <a href='donate'>
                            Donate
                            </a>
                            <p>@StillNeedsAname, All rights reserved</p>
                        </div>
                    </footer>"
                    );
                    let mut file = File::open("static/serps/web.html").unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let statistics = format!("<div style='position: fixed; right: 10px; margin: 0; padding: 0;'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, duration.as_millis());
                    let search_box_statistics = format!("Search results for '{}'", query);
                    contents = contents.replace("{{statistics}}", &statistics).replace("{{search_box_statistics}}", &search_box_statistics);
                    search_results = search_results.replace("\", ", "").replace("\",", "").replace("\"", "").replace(">[<", "><").replace(">]<", "");
                    contents = format!("{}{}{}", contents, search_results, search_results_footer);
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

pub fn tantivy_processor(query: &str) -> tantivy::Result<(Vec<String>, Duration)> {

    // Define the schema and how to index it
    let index_dir = "index";
    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let description = schema_builder.add_text_field("description", TEXT | STORED);
    let author = schema_builder.add_text_field("author", TEXT | STORED);
    let year_published = schema_builder.add_i64_field("year_published", STORED | FAST);
    let year_updated = schema_builder.add_i64_field("year_updated", STORED | FAST);
    let url = schema_builder.add_text_field("clearnet", TEXT | STORED);
    let onion = schema_builder.add_text_field("onion", TEXT | STORED);
    let status = schema_builder.add_text_field("status", TEXT | STORED);
    let parent_organization = schema_builder.add_text_field("parent_organization", TEXT | STORED);
    let sub_organization = schema_builder.add_text_field("sub_organization", TEXT | STORED);
    let subject_type = schema_builder.add_text_field("subject_type", TEXT | STORED);
    let nonprofit_status = schema_builder.add_text_field("nonprofit_status", TEXT | STORED);
    let identifier = schema_builder.add_text_field("identifier", TEXT | STORED);
    let category = schema_builder.add_text_field("category", TEXT | STORED | FAST);
    let tags = schema_builder.add_text_field("tags", TEXT | STORED | FAST);
    let instant_answer = schema_builder.add_i64_field("instant_answer", STORED | FAST);
    let schema = schema_builder.build();
    let index = if fs::read_dir(index_dir)?.count() == 0 {

    // Create the index
    let index = Index::create_in_dir("index", schema.clone())?;

    // Parse the TOML files
    let paths = fs::read_dir("datasets")?;
    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let data = fs::read_to_string(path)
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
                index_writer.commit()?;
            }
        }
    }
    println!("Indexing complete!");
    index
    } else {
        Index::open_in_dir(index_dir)?
    };

    // Searching the index
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let start = Instant::now();


    let searcher = reader.searcher();

    // Querying the index
    let query_parser = QueryParser::for_index(&index, vec![title, description, author, category, tags, url]);
    
    // Split the query into words
    let words: Vec<&str> = query.split_whitespace().collect();

    // Separate the words to include, exclude, and mandatory
    let (include_words, exclude_words, mandatory_words, field_queries): (Vec<&str>, Vec<&str>, Vec<&str>, Vec<(String, String)>) = words.iter().fold((Vec::new(), Vec::new(), Vec::new(), Vec::new()), |(mut include, mut exclude, mut mandatory, mut field), &word| {
        if word.starts_with('-') {
            exclude.push(&word[1..]);
        } else if word.starts_with('+') {
            mandatory.push(&word[1..]);
        } else if word.contains(':') {
            let mut split = word.splitn(2, ':');
            let field_name = split.next().unwrap().to_string();
            let field_value = split.next().unwrap_or("").to_string();
            field.push((field_name, field_value));
        } else {
            include.push(word);
        }
        (include, exclude, mandatory, field)
    });

    // Create a query for the words to include
    let include_query = if !include_words.is_empty() {
        Some(query_parser.parse_query(&include_words.join(" "))?)
    } else {
        None
    };

    // Create a query for the words to exclude
    let exclude_query = if !exclude_words.is_empty() {
        Some(query_parser.parse_query(&exclude_words.join(" "))?)
    } else {
        None
    };

    // Create a query for the mandatory words
    let mandatory_query = if !mandatory_words.is_empty() {
        Some(query_parser.parse_query(&mandatory_words.join(" "))?)
    } else {
        None
    };

    // Combine the queries
    let mut final_query: Vec<(Occur, Box<dyn Query>)> = Vec::new();

    if let Some(include_query) = include_query {
        final_query.push((Occur::Should, include_query));
    }

    if let Some(exclude_query) = exclude_query {
        final_query.push((Occur::MustNot, exclude_query));
    }

    if let Some(mandatory_query) = mandatory_query {
        final_query.push((Occur::Must, mandatory_query));
    }

    for (field_name, field_value) in field_queries {
        let field = schema.get_field(&field_name).map_err(|_| tantivy::TantivyError::SchemaError(format!("Unknown field: {}", field_name)))?;
        let term = Term::from_field_text(field, &field_value);
        let field_query = TermQuery::new(term, IndexRecordOption::Basic);
        final_query.push((Occur::Must, Box::new(field_query)));
    }

    let final_query = BooleanQuery::from(final_query);

    // Collectors
    let top_docs = searcher.search(&final_query, &TopDocs::with_limit(50))?;
    let mut scored_results = Vec::new();
    for (mut score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let html = document_to_html(&retrieved_doc, &schema);
        let html_without_quotes = html.replace(", ", "");

        // Modify the search ranking based on the content
        if let Some(tantivy::schema::Value::Str(url)) = retrieved_doc.get_first(url) {
            if url.contains("reddit.com") || url.contains("quora.com") || url.contains("youtube.com/shorts") {
                score *= 0.5; // Decrease score by 50%
            } else if url.contains("onlyfans.com") || url.contains("fansly.com") || url.contains("linktr.ee") || url.contains("t.me") {
                score *= 0.9; // Decrease score by 90%
            }
        }
        if let Some(tantivy::schema::Value::Str(instant_answer)) = retrieved_doc.get_first(instant_answer) {
            if !instant_answer.is_empty() {
                score *= 100.0; // Increase by 100x
            }
        }
        //if let Some(tantivy::schema::Value::Str(title)) = retrieved_doc.get_first(title) {
        //    if title.contains("Linux") {
        //        score *= 5.0; // Increase score by 500%
        //   }
        //}
        scored_results.push((score, html_without_quotes));
    }

// Sort the results by score in descending order
scored_results.sort_by(|(score1, _), (score2, _)| score2.partial_cmp(score1).unwrap());

// Extract the html_without_quotes for each pair
let results: Vec<String> = scored_results.into_iter().map(|(_, html)| html).collect();

let duration = start.elapsed();
Ok((results, duration))
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
    let title = if title.len() > 200 {
        format!("{}...", &title.chars().take(100).collect::<String>())
    } else {
        title
    };
    let description = fields.get("description").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");
    let description = if description.len() > 200 {
        format!("{}...", &description.chars().take(200).collect::<String>())
    } else {
        description
    };
    let author = fields.get("author").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");
    let author_html = if !author.is_empty() {
        format!("<a class='pill' href='/search?q={}'>author: {}</a>", author, author)
    } else {
        "".to_string()
    };
    let tags = fields.get("tags").unwrap_or_else(|| &&default_value);
    let tags_html = if !tags.is_empty() {
        format!("<a class='pill' href='/search?q={}'>{}</a>", tags.to_lowercase(), tags.to_lowercase())
    } else {
        "".to_string()
    };
    let category = fields.get("category").unwrap_or_else(|| &&default_value).replace(", ", "");
    let category_html = if !category.is_empty() {
        format!("<a class='pill' href='/search?q={}'>{}</a>", category.to_lowercase(), category.to_lowercase())
    } else {
        "".to_string()
    };
    let url = fields.get("clearnet").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");
    let onion = fields.get("onion").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");

    let url_html = match (onion.is_empty(), url.is_empty()) {
        (false, false) => format!("<p class='search-result-url-row'><a class='search-result-url' href='{}'>Onion</a> | {}</p>", onion, url),
        (false, true) => format!("<p class='search-result-url-row'><a class='search-result-url' href='{}'>Onion</a></p>", onion),
        (true, false) => format!("<p class='search-result-url-row'>{}</p>", url),
        (true, true) => "".to_string(),
    };

    let instant_answer = fields.get("instant_answer").unwrap_or_else(|| &&default_value).replace("[", "").split(',').collect::<Vec<&str>>().join("");

    let result_html = if instant_answer == "sports" {
        format!("<details class='more-info'><summary>{}</summary><div class='search-result'><h3><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></h3><p class='sports-description'>{}</p><div class='sports-related-row'>Related: {}{}{}</div></div></details>",
        title, url, title, description, author_html, category_html, tags_html)
    } else {
        format!("<div class='search-result'>{}<h3><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></h3><p class='search-result-description'>{}</p><div class='search-result-related-row'>Related: {}{}{}{}</div></div>",
        url_html, url, title, description, author_html, category_html, tags_html, instant_answer)
    };

    result_html
}