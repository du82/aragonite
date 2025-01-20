// search.rs

use rocket::form::{Form, FromForm};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::uri;
use rocket::{get, post};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::BooleanQuery;
use tantivy::query::Occur;
use tantivy::query::Query;
use tantivy::query::QueryParser;
use tantivy::query::TermQuery;
use tantivy::schema::IndexRecordOption;
use tantivy::schema::{Schema, FAST, STORED, TEXT};
use tantivy::Document;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tantivy::Term;
use toml::Value;
use crate::common::url_utils::{extract_domain, is_homepage, get_domain_name};
use crate::common::document_to_html::document_to_html;

#[derive(FromForm)]
struct SearchForm {
    query: String,
}

use std::time::SystemTime;

#[get("/random")]
pub fn random_search() -> Result<Redirect, std::io::Error> {
    let content = fs::read_to_string("random.txt")?;
    let queries: Vec<&str> = content.lines().collect();
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let query = queries[(time as usize) % queries.len()];
    Ok(Redirect::to(uri!(search_get(q = Some(query.to_string())))))
}

#[post("/search", data = "<search_form>")]
pub fn search_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(search_get(q = Some(search_form.into_inner().query))))
}

#[get("/search?<q>")]
pub fn search_get(q: Option<String>) -> Result<RawHtml<String>, Redirect> {
    match tantivy_processor(q.as_deref().unwrap_or("")) {
        Ok((results, duration)) => {
            //println!("{:#?}", results);
            match q {
                Some(query) => {
                    let trimmed_query = query.trim_start();
                    if trimmed_query.len() < 1 || query.len() > 128 {
                        return Ok(RawHtml(
                            "<h1>Error: Search query must be between 1 and 128 characters long.</h1>"
                                .to_string(),
                        ));
                    }
                    let num_results = results.len();
                    let result_word = if num_results == 1 {
                        "result"
                    } else {
                        "results"
                    };
                    let mut search_results =
                        format!("<div class='results-container'>{:?}</div>", results);
                    let search_results_footer = format!(
                        "<div class='main-footer'>
                        <div class='footer-container'>
                            <a href='donate'>
                            Donate
                            </a>|
                            <a href='mirrors'>
                            Mirrors
                            </a>|
                            <a href='privacy'>
                            Privacy
                            </a>|
                            <a href='settings'>
                            Settings </a>
                            <p class='centered-text'>
                            Don't be Hypocrites
                            </p>
                        </div>
                        </div>"
                    );
                    let mut file = File::open("static/serps/web.html").unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let statistics = format!("<div class='statistics'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, duration.as_millis());
                    let search_box_statistics = format!("Search results for '{}'", query);
                    contents = contents
                        .replace("{{statistics}}", &statistics)
                        .replace("{{search_box_statistics}}", &search_box_statistics);
                    search_results = search_results
                        .replace("\", ", "")
                        .replace("\",", "")
                        .replace("\"", "")
                        .replace(">[<", "><")
                        .replace(">]<", "");
                    contents = format!("{}{}{}", contents, search_results, search_results_footer);
                    Ok(RawHtml(contents))
                }
                None => Err(Redirect::to("/")),
            }
        }
        Err(e) => {
            println!("Error creating index: {:?}", e);
            Err(Redirect::to("/"))
        }
    }
}

#[post("/mobile", data = "<search_form>")]
pub fn mobile_search_post(search_form: Form<SearchForm>) -> Redirect {
    Redirect::to(uri!(mobile_search_get(q = Some(search_form.into_inner().query))))
}

#[get("/mobile?<q>")]
pub fn mobile_search_get(q: Option<String>) -> Result<RawHtml<String>, Redirect> {
    match tantivy_processor(q.as_deref().unwrap_or("")) {
        Ok((results, duration)) => {
            //println!("{:#?}", results);
            match q {
                Some(query) => {
                    let trimmed_query = query.trim_start();
                    if trimmed_query.len() < 1 || query.len() > 128 {
                        return Ok(RawHtml(
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
                    let mut search_results =
                        format!("<div class='results-container'>{:?}</div>", results);
                    let search_results_footer = format!(
                        "<div class='main-footer'>
                        <div class='footer-container'>
                            <a href='about'>
                            About Us
                            </a>|
                            <a href='privacy'>
                            Privacy Policy
                            </a>|
                            <a href='jobs'>
                            Careers
                            </a>|
                            <a href='donate'>
                            Donate
                            </a>|
                            <a href='settings'>
                            Settings
                            </a>
                            <p>Don't be Hypocrites // <b>2024 Aragonite</b>, Some rights reserved</p>
                        </div>
                        </div>"
                    );
                    let mut file = File::open("static/serps/mobile.html").unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let statistics = format!("<div class='statistics'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, duration.as_millis());
                    let search_box_statistics = format!("Search results for '{}'", query);
                    contents = contents
                        .replace("{{statistics}}", &statistics)
                        .replace("{{search_box_statistics}}", &search_box_statistics);
                    search_results = search_results
                        .replace("\", ", "")
                        .replace("\",", "")
                        .replace("\"", "")
                        .replace(">[<", "><")
                        .replace(">]<", "");
                    contents = format!("{}{}{}", contents, search_results, search_results_footer);
                    Ok(RawHtml(contents))
                }
                None => Err(Redirect::to("/")),
            }
        }
        Err(e) => {
            println!("Error creating index: {:?}", e);
            Err(Redirect::to("/"))
        }
    }
}

pub fn tantivy_processor(query: &str) -> tantivy::Result<(Vec<String>, Duration)> {
    // Define the schema and how to index it
    let index_dir = "index";
    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let description = schema_builder.add_text_field("description", TEXT | STORED);
    let full_description = schema_builder.add_text_field("full_description", TEXT | STORED);
    let author = schema_builder.add_text_field("author", TEXT | STORED);
    let _year_published = schema_builder.add_i64_field("year_published", STORED | FAST);
    let _year_updated = schema_builder.add_i64_field("year_updated", STORED | FAST);
    let url = schema_builder.add_text_field("clearnet", TEXT | STORED);
    let _onion = schema_builder.add_text_field("onion", TEXT | STORED);
    let _status = schema_builder.add_text_field("status", TEXT | STORED);
    let _parent_organization = schema_builder.add_text_field("parent_organization", TEXT | STORED);
    let _sub_organization = schema_builder.add_text_field("sub_organization", TEXT | STORED);
    let _jurisdiction = schema_builder.add_text_field("jurisdiction", TEXT | STORED);
    let _jurisdiction_level = schema_builder.add_text_field("jurisdiction_level", TEXT | STORED);
    let _regulates_entity = schema_builder.add_text_field("regulates_entity", TEXT | STORED);
    let _regulator_entity = schema_builder.add_text_field("regulator_entity", TEXT | STORED);
    let subject_type = schema_builder.add_text_field("subject_type", TEXT | STORED);
    let _nonprofit_status = schema_builder.add_text_field("nonprofit_status", TEXT | STORED);
    let _religious_affiliation = schema_builder.add_text_field("religious_affiliation", TEXT | STORED);
    let category = schema_builder.add_text_field("category", TEXT | STORED | FAST);
    let tags = schema_builder.add_text_field("tags", TEXT | STORED | FAST);
    let privacy_track_record = schema_builder.add_text_field("ptr", TEXT | STORED);
    let human_rights_track_record = schema_builder.add_text_field("hrr", TEXT | STORED);
    let instant_answer = schema_builder.add_i64_field("instant_answer", STORED | FAST);
    let _language = schema_builder.add_text_field("language", TEXT | STORED);
    let schema = schema_builder.build();

    let index = if fs::read_dir(index_dir)?.count() == 0 {
        // Create the index
        let index = Index::create_in_dir("index", schema.clone())?;

        // Create a single writer for all documents
        let mut index_writer = index.writer(50_000_000)?;

        // Parse the TOML files recursively
        let mut dirs_to_process = vec![std::path::PathBuf::from("datasets")];

        while let Some(dir) = dirs_to_process.pop() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    dirs_to_process.push(path);
                    continue;
                }

                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    let data = fs::read_to_string(&path)
                        .map_err(|e| tantivy::TantivyError::InvalidArgument(format!("{}", e)))?;
                    let value: Value = toml::from_str(&data)
                        .map_err(|e| tantivy::TantivyError::InvalidArgument(format!("{}", e)))?;

                    if let Some(tables) = value.as_table() {
                        for (_name, table) in tables {
                            let mut doc = Document::new();

                            // Add the relative path as source for namespacing
                            if let Ok(source_field) = schema.get_field("source") {
                                if let Ok(rel_path) = path.strip_prefix("datasets") {
                                    if let Some(parent) = rel_path.parent() {
                                        if let Some(path_str) = parent.to_str() {
                                            doc.add_text(source_field, path_str);
                                        }
                                    }
                                }
                            }

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
                            index_writer.add_document(doc)?;
                        }
                    }
                }
            }
        }

        // Single commit at the end
        index_writer.commit()?;
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
    let query_parser = QueryParser::for_index(
        &index,
        vec![
            title,
            description,
            full_description,
            author,
            subject_type,
            category,
            tags,
            url,
        ],
    );

    // Split the query into words
    let words: Vec<&str> = query.split_whitespace().collect();

    // Separate the words to include, exclude, and mandatory
    let (include_words, exclude_words, mandatory_words, field_queries): (
        Vec<&str>,
        Vec<&str>,
        Vec<&str>,
        Vec<(String, String)>,
    ) = words.iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |(mut include, mut exclude, mut mandatory, mut field), &word| {
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
        },
    );

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
        let field = schema.get_field(&field_name).map_err(|_| {
            tantivy::TantivyError::SchemaError(format!("Unknown field: {}", field_name))
        })?;
        let term = Term::from_field_text(field, &field_value);
        let field_query = TermQuery::new(term, IndexRecordOption::Basic);
        final_query.push((Occur::Must, Box::new(field_query)));
    }

    // Output of the final query after parsing
    let final_query = BooleanQuery::from(final_query);

    let config_str = fs::read_to_string("Config.toml").unwrap();
    let config: Value = config_str.parse().unwrap();

    // Get the number of results per query from Config.toml
    let results_per_query = config.get("engine")
        .and_then(|engine| engine.get("results_per_query"))
        .and_then(Value::as_integer)
        .unwrap_or(50) as usize;

    // Collectors
    let top_docs = searcher.search(&final_query, &TopDocs::with_limit(results_per_query))?;
    let mut scored_results = Vec::new();
    for (mut score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let html = document_to_html(&retrieved_doc, &schema);
        let html_without_quotes = html.replace(", ", "");

        // Downrank pages without title
        if let Some(tantivy::schema::Value::Str(title)) = retrieved_doc.get_first(title) {
            if title.trim().is_empty() {
                score *= 0.3;
            }
        } else {
            score *= 0.3;
        }

        // URL-based scoring using url_utils functions
        if let Some(tantivy::schema::Value::Str(url_str)) = retrieved_doc.get_first(url) {
            let domain = get_domain_name(url_str);

            // Check if it's a homepage which might deserve different ranking
            if is_homepage(url_str) {
                score *= 1.2; // Small boost for homepages
            }

            // Use extracted domain for ranking decisions
            if let Some(domain) = domain {
                if domain == "reddit.com" || domain == "quora.com" || domain.contains("youtube.com") {
                    score *= 0.5;
                } else if domain == "onlyfans.com" || domain == "fansly.com" ||
                          domain == "linktr.ee" || domain == "t.me" {
                    score *= 0.1;
                }
            }
        }

        // Handle description length-based scoring
        if let Some(tantivy::schema::Value::Str(desc)) = retrieved_doc.get_first(description) {
            let desc_len = desc.trim().len();
            if desc_len == 0 {
                score *= 0.4;
            } else {
                let length_multiplier = match desc_len {
                    0..=49 => 0.8,
                    50..=150 => 1.0,
                    _ => 1.5,
                };
                score *= length_multiplier;
            }
        } else {
            score *= 0.4;
        }

        // Other ranking factors remain unchanged
        if let Some(tantivy::schema::Value::Str(instant_answer)) = retrieved_doc.get_first(instant_answer) {
            if !instant_answer.is_empty() {
                score *= 100.0;
            }
        }

        if let Some(tantivy::schema::Value::Str(privacy_track_record)) = retrieved_doc.get_first(privacy_track_record) {
            if privacy_track_record == "excellent" {
                score *= 1.2;
            } else if privacy_track_record == "good" {
                score *= 1.1;
            } else if privacy_track_record == "bad" {
                score *= 0.9;
            } else if privacy_track_record == "terrible" {
                score *= 0.8;
            }
        }

        if let Some(tantivy::schema::Value::Str(human_rights_track_record)) = retrieved_doc.get_first(human_rights_track_record) {
            if human_rights_track_record == "excellent" {
                score *= 1.2;
            } else if human_rights_track_record == "good" {
                score *= 1.1;
            } else if human_rights_track_record == "bad" {
                score *= 0.9;
            } else if human_rights_track_record == "terrible" {
                score *= 0.8;
            }
        }

        scored_results.push((score, html_without_quotes));
    }

    // Sort the results by score in descending order
    scored_results.sort_by(|(score1, _), (score2, _)| score2.partial_cmp(score1).unwrap());

    // Extract the html_without_quotes for each pair
    let results: Vec<String> = scored_results.into_iter().map(|(_, html)| html).collect();

    let duration = start.elapsed();
    Ok((results, duration))
}
