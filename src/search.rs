// Search functionality implementation

use crate::benchmarks::search_benchmark;
use crate::common::document_to_html::document_to_html;
use crate::common::url_utils::{get_domain_name, is_homepage};
use crate::common::url_utils::extract_domain;
use crate::filter::FilterManager;
use rocket::form::{Form, FromForm};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::uri;
use std::sync::OnceLock;
use rocket::{get, post, State};
use rocket::http::{Cookie, CookieJar};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
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
use regex::Regex;

#[derive(FromForm)]
pub struct SearchForm {
    query: String,
}

use std::time::SystemTime;


#[get("/random")]
pub fn random_search(cookies: &CookieJar<'_>) -> Result<Redirect, std::io::Error> {
    let content = fs::read_to_string("random.txt")?;
    let queries: Vec<&str> = content.lines().collect();
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let query = queries[(time as usize) % queries.len()];

    // Get current safe search level from cookie
    let ss_level = cookies.get("safe_search_level")
        .map(|cookie| match cookie.value() {
            "off" => "0",
            "moderate" => "1",
            "strict" => "2",
            _ => "1" // Default to moderate
        })
        .unwrap_or("1"); // Default to moderate if no cookie

    Ok(Redirect::to(uri!(search_get(q = Some(query.to_string()), ss = Some(ss_level.to_string())))))
}

fn sanitize_query(query: &str) -> String {
    let special_chars = &['^', '[', ']', '{', '}', '(', ')', '\\', '*', '+', '|', '&', '!', ':', '~', '>', '<'];

    let mut sanitized = query.to_string();
    for &c in special_chars {
        sanitized = sanitized.replace(c, "");
    }

    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(&sanitized, " ").trim().to_string()
}

#[post("/search", data = "<search_form>")]
pub fn search_post(search_form: Form<SearchForm>, cookies: &CookieJar<'_>) -> Redirect {
    // Get current safe search level from cookie to maintain it in the redirect
    let ss_level = cookies.get("safe_search_level")
        .map(|cookie| match cookie.value() {
            "off" => "0",
            "moderate" => "1",
            "strict" => "2",
            _ => "0" // Default to moderate
        })
        .unwrap_or("0"); // Default to off if no cookie

    let sanitized_query = sanitize_query(&search_form.into_inner().query);
    // Only pass the query if it's not empty after sanitization
    if sanitized_query.trim().is_empty() {
        Redirect::to(uri!(search_get(q = None::<String>, ss = Some(ss_level.to_string()))))
    } else {
        Redirect::to(uri!(search_get(q = Some(sanitized_query), ss = Some(ss_level.to_string()))))
    }
}

#[get("/search?<q>&<ss>")]
pub fn search_get(q: Option<String>, ss: Option<String>, cookies: &CookieJar<'_>, filter_manager: &State<Mutex<FilterManager>>) -> Result<RawHtml<String>, Redirect> {
    // Sanitize query and filter out empty results
    let sanitized_q = q.as_ref().map(|query| {
        let sanitized = sanitize_query(query);
        if sanitized.trim().is_empty() { None } else { Some(sanitized) }
    }).flatten();
    // Process safe search level if provided
    if let Some(level) = ss {
        let valid_level = match level.as_str() {
            "0" => "off",
            "1" => "moderate",
            "2" => "strict",
            _ => "moderate", // Default to moderate if invalid
        };

        cookies.add(Cookie::new("safe_search_level", valid_level.to_string()));

        let mut filter = filter_manager.lock().unwrap();
        filter.set_level(valid_level);
    }

    // Handle admin commands
    if let Some(query) = q.as_deref() {
        if query == "--run-benchmark" {
            let benchmark_results = run_benchmarks();
            return Ok(RawHtml(benchmark_results));
        } else if query == "--inspect-dataset" {
            let (valid, invalid, details) = inspect_dataset_directory();

            let report = format!(
                "<div style='max-width: 800px; margin: 50px auto; padding: 20px; border: 1px solid #ddd; border-radius: 5px;'>
                <h1>Dataset Inspection Report</h1>
                <p><strong>Valid TOML files:</strong> {}</p>
                <p><strong>Invalid TOML files:</strong> {}</p>
                <div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #d32f2f; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>
                {}
                </div>
                <p>Run a full inspection by using: <code>find datasets -name \"*.toml\" -exec cat \\\\ \\; | head -1</code> to check the first line of each file</p>
                <p><a href='/' style='color: #1976d2; text-decoration: none;'>Return to homepage</a></p>
                </div>",
                valid, invalid, details
            );
            return Ok(RawHtml(report));
        } else if query == "--find-invalid-files" {
            let report = find_invalid_dataset_files();
            return Ok(RawHtml(report));
        }
    }

    let filter = filter_manager.lock().unwrap();

    // If sanitized query is None (empty after sanitization), return empty results
    if sanitized_q.is_none() && q.is_some() {
        // Return empty results template rather than error for visually empty queries
        return Ok(RawHtml(format!(
            "<div class='results-container'><p>No results found</p></div>
            <div class='main-footer'>
                <div class='footer-container'>
                    <a href='donate'>Donate</a>|
                    <a href='mirrors'>Mirrors</a>|
                    <a href='privacy'>Privacy</a>|
                    <a href='settings'>Settings</a>
                </div>
            </div>"
        )));
    }

    match tantivy_processor(sanitized_q.as_deref().unwrap_or(""), &filter) {
        Ok((results, duration)) => {
            match sanitized_q {
                Some(query) => {
                    // Only check length for non-empty queries
                    if query.len() > 128 {
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

                    // Check if this is a filtered content notice
                    let is_filtered_notice = results.len() == 1 && results[0].contains("filtered-notice");
                    let mut search_results = if is_filtered_notice {
                        // For filtered content, don't use the debug formatting
                        format!("<div class='results-container'>{}</div>", results[0])
                    } else {
                        format!("<div class='results-container'>{:?}</div>", results)
                    };
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
                            Aragonite 2025
                            </p>
                        </div>
                        </div>"
                    );
                    let mut file = File::open("static/serps/web.html").unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    // Use the already locked filter for statistics
                    let statistics = if is_filtered_notice {
                        format!("<div class='statistics'><span><h3>Content filtered</h3><p>(safe search is enabled - level: {})</p></span></div>", filter.get_level())
                    } else {
                        format!("<div class='statistics'><span><h3>{} {} for: {}</h3><p>(retrieved in {} ms)</p></span></div>", num_results, result_word, query, duration.as_millis())
                    };
                    let search_box_statistics = if is_filtered_notice {
                        format!("Search for '{}' (filtered)", query)
                    } else {
                        format!("Search results for '{}'", query)
                    };
                    contents = contents
                        .replace("{{statistics}}", &statistics)
                        .replace("{{search_box_statistics}}", &search_box_statistics);

                    // Only apply these replacements if it's not a filtered notice
                    if !is_filtered_notice {
                        search_results = search_results
                            .replace("\", ", "")
                            .replace("\",", "")
                            .replace("\"", "")
                            .replace(">[<", "><");
                    }

                    search_results = search_results.replace(">]<", "");
                    contents = format!("{}{}{}", contents, search_results, search_results_footer);
                    Ok(RawHtml(contents))
                }
                None => Err(Redirect::to("/")),
            }
        }
        Err(e) => {
            eprintln!("Error creating index: {:?}", e);

            // Format the error message in a more user-friendly way
            let error_html = format!(
                "<div style='max-width: 800px; margin: 50px auto; padding: 20px; border: 1px solid #ddd; border-radius: 5px;'>
                    <h1 style='color: #d32f2f;'>Error Creating Search Index</h1>
                    <p style='margin-bottom: 20px;'>An error occurred while creating or accessing the search index:</p>
                    <div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #d32f2f; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>
                        {}
                    </div>
                    <p style='margin-top: 20px;'>Please check the server logs for more details. This could be caused by:</p>
                    <ul>
                        <li>Invalid TOML files in the datasets directory</li>
                        <li>XML or HTML files mistakenly saved with .toml extension</li>
                        <li>Permission issues with the index directory</li>
                        <li>Disk space limitations</li>
                    </ul>
                    <p><a href='/' style='color: #1976d2; text-decoration: none;'>Return to homepage</a></p>
                </div>",
                e
            );

            Ok(RawHtml(error_html))
        }
    }
}

/// A utility function to inspect the dataset directory and check for invalid TOML files
/// Returns a tuple with (number of valid files, number of invalid files, details of first few errors)
pub fn inspect_dataset_directory() -> (usize, usize, String) {
    let mut valid_files = 0;
    let mut invalid_files = 0;
    let mut error_details = String::new();
    let mut first_few_errors = Vec::new();

    let mut dirs_to_process = vec![std::path::PathBuf::from("datasets")];

    while let Some(dir) = dirs_to_process.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        dirs_to_process.push(path);
                        continue;
                    }

                    if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                        // Read the file once
                        let data = match fs::read_to_string(&path) {
                            Ok(content) => content,
                            Err(e) => {
                                invalid_files += 1;
                                let error_msg = format!("File '{}': Error reading file: {}", path.display(), e);
                                if first_few_errors.len() < 5 {
                                    first_few_errors.push(error_msg);
                                }
                                continue;
                            }
                        };

                        // Validate TOML content
                        match validate_toml_content(&data) {
                            Ok(_) => {
                                valid_files += 1;
                            },
                            Err(e) => {
                                invalid_files += 1;
                                let error_msg = format!("File '{}': Invalid TOML syntax: {}", path.display(), e);
                                if first_few_errors.len() < 5 {
                                    first_few_errors.push(error_msg);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Format error details
    if !first_few_errors.is_empty() {
        error_details = format!("First {} errors:\n{}",
            first_few_errors.len(),
            first_few_errors.join("\n")
        );
    }

    (valid_files, invalid_files, error_details)
}

/// A more comprehensive utility to find and report invalid files in the dataset
pub fn find_invalid_dataset_files() -> String {
    use std::process::Command;

    let mut html = String::from(
        "<div style='max-width: 800px; margin: 50px auto; padding: 20px; border: 1px solid #ddd; border-radius: 5px;'>
            <h1>Invalid Dataset Files Report</h1>"
    );

    // Count total files first
    let total_files = if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("find datasets -name \"*.toml\" -type f | wc -l")
        .output()
    {
        String::from_utf8_lossy(&output.stdout).trim().parse::<usize>().unwrap_or(0)
    } else {
        0
    };

    html.push_str(&format!("<p>Total TOML files found: {}</p>", total_files));

    // First, try to find files that start with '<' character (likely XML/HTML)
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("find datasets -name \"*.toml\" -type f -exec grep -l \"^<\" {} \\;")
        .output()
    {
        let xml_like_files = String::from_utf8_lossy(&output.stdout);
        let count = xml_like_files.lines().count();

        html.push_str(&format!(
            "<h2>Files with XML-like content ({}) - {}%</h2>",
            count,
            if total_files > 0 { (count * 100) / total_files } else { 0 }
        ));

        if count > 0 {
            html.push_str("<div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #e57373; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>");
            for file in xml_like_files.lines().take(20) {
                html.push_str(&format!("{}<br>", file));
            }
            if count > 20 {
                html.push_str(&format!("... and {} more files", count - 20));
            }
            html.push_str("</div>");

            // For the first file, show a preview
            if let Some(first_file) = xml_like_files.lines().next() {
                if let Ok(preview_output) = Command::new("sh")
                    .arg("-c")
                    .arg(&format!("head -n 5 \"{}\"", first_file))
                    .output()
                {
                    let preview = String::from_utf8_lossy(&preview_output.stdout);
                    html.push_str("<h3>Preview of first problematic file:</h3>");
                    html.push_str("<div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #9575cd; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>");
                    html.push_str(&format!("{}</div>", preview.replace("<", "&lt;").replace(">", "&gt;")));
                }
            }
        } else {
            html.push_str("<p>No XML-like files found.</p>");
        }
    }

    // Look for JSON-like files (starting with '{')
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("find datasets -name \"*.toml\" -type f -exec grep -l \"^{\" {} \\;")
        .output()
    {
        let json_like_files = String::from_utf8_lossy(&output.stdout);
        let count = json_like_files.lines().count();

        html.push_str(&format!(
            "<h2>Files with JSON-like content ({})</h2>", count
        ));

        if count > 0 {
            html.push_str("<div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #4fc3f7; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>");
            for file in json_like_files.lines().take(20) {
                html.push_str(&format!("{}<br>", file));
            }
            if count > 20 {
                html.push_str(&format!("... and {} more files", count - 20));
            }
            html.push_str("</div>");
        } else {
            html.push_str("<p>No JSON-like files found.</p>");
        }
    }

    // Find empty files
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("find datasets -name \"*.toml\" -type f -size 0")
        .output()
    {
        let empty_files = String::from_utf8_lossy(&output.stdout);
        let count = empty_files.lines().count();

        html.push_str(&format!(
            "<h2>Empty files ({})</h2>", count
        ));

        if count > 0 {
            html.push_str("<div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #4db6ac; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>");
            for file in empty_files.lines() {
                html.push_str(&format!("{}<br>", file));
            }
            html.push_str("</div>");
        } else {
            html.push_str("<p>No empty files found.</p>");
        }
    }

    // Find very large files (>1MB)
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("find datasets -name \"*.toml\" -type f -size +1M")
        .output()
    {
        let large_files = String::from_utf8_lossy(&output.stdout);
        let count = large_files.lines().count();

        html.push_str(&format!(
            "<h2>Very large files (>1MB) ({})</h2>", count
        ));

        if count > 0 {
            html.push_str("<div style='background: #f8f8f8; padding: 15px; border-left: 5px solid #ffb74d; font-family: monospace; white-space: pre-wrap; overflow-wrap: break-word;'>");
            for file in large_files.lines() {
                html.push_str(&format!("{}<br>", file));
            }
            html.push_str("</div>");
        } else {
            html.push_str("<p>No very large files found.</p>");
        }
    }

    html.push_str("<h2>Fix Commands</h2>");

    html.push_str("<p>If you found XML-like files, you can remove them with:</p>");
    html.push_str("<div style='background: #f8f8f8; padding: 15px; font-family: monospace;'>");
    html.push_str("find datasets -name \"*.toml\" -type f -exec grep -l \"^<\" {} \\; | xargs rm</div>");

    html.push_str("<p>If you found JSON-like files, you can remove them with:</p>");
    html.push_str("<div style='background: #f8f8f8; padding: 15px; font-family: monospace;'>");
    html.push_str("find datasets -name \"*.toml\" -type f -exec grep -l \"^{\" {} \\; | xargs rm</div>");

    html.push_str("<p>Check for UTF-8 BOM markers (common in Windows-edited files):</p>");
    html.push_str("<div style='background: #f8f8f8; padding: 15px; font-family: monospace;'>");
    html.push_str("find datasets -name \"*.toml\" -type f -exec hexdump -n 3 {} \\; | grep \"ef bb bf\"</div>");

    html.push_str("<p>Fix BOM markers if present:</p>");
    html.push_str("<div style='background: #f8f8f8; padding: 15px; font-family: monospace;'>");
    html.push_str("for f in $(find datasets -name \"*.toml\" -type f); do sed -i 's/\\\\xEF\\\\xBB\\\\xBF//g' \"$f\"; done</div>");

    html.push_str("<p><a href='/' style='color: #1976d2; text-decoration: none;'>Return to homepage</a></p>");
    html.push_str("</div>");

    html
}

pub fn run_benchmarks() -> String {
    match search_benchmark::run_search_benchmark() {
        Ok(_) => match std::fs::read_to_string("benchmark_results.txt") {
            Ok(contents) => format!(
                "<pre style='white-space: pre-wrap; font-family: monospace;'>{}</pre>",
                contents
            ),
            Err(e) => format!("Error reading benchmark results: {}", e),
        },
        Err(e) => format!("Error running benchmarks: {}", e),
    }
}

/// Validates if a file contains valid TOML content without reading the file again
/// Validates TOML content from a string without reading the file again
fn validate_toml_content(data: &str) -> Result<(), String> {
    // Get a preview of the content for error reporting
    let preview = if data.len() > 100 {
        format!("{:.100}...", data.replace('\n', "\\n"))
    } else {
        data.replace('\n', "\\n")
    };

    // Check for empty files
    if data.trim().is_empty() {
        return Err(format!("File is empty or contains only whitespace"));
    }

    // Check for common content mistaken for TOML
    if data.trim().starts_with('<') {
        if data.contains("</") {
            return Err(format!("File appears to contain XML/HTML, not TOML. Preview: {}", preview));
        } else {
            return Err(format!("File starts with '<' character, possibly XML/HTML. Preview: {}", preview));
        }
    }

    if data.trim().starts_with('{') && data.contains('}') {
        return Err(format!("File appears to contain JSON, not TOML. Preview: {}", preview));
    }

    // Check for file size issues
    if data.len() > 10_000_000 {  // 10MB
        return Err(format!("File is too large ({} bytes)", data.len()));
    }

    if data.starts_with("\u{feff}") {
        return Err(format!("File contains UTF-8 BOM marker which is not supported by TOML"));
    }

    // Attempt to parse as TOML
    match toml::from_str::<Value>(data) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Invalid TOML syntax: {}. Preview: {}", e, preview)),
    }
}

pub fn tantivy_processor(query: &str, filter_manager: &FilterManager) -> tantivy::Result<(Vec<String>, Duration)> {
    let sanitized_query = sanitize_query(query);
    if filter_manager.is_enabled() && filter_manager.should_filter_query(&sanitized_query) {
        // Return a special message informing the user about filtered content
        let filtered_notice = vec![
            format!("<div class='search-result filtered-notice' style='background-color: rgba(255, 82, 82, 0.1); border-left: 4px solid #ff5252; padding: 15px;'>
                <h3 style='margin-top: 0; color: #ff5252;'>Content Filtered</h3>
                <div class='search-result-description'>
                    <p>Your search term contains content that is filtered by the safe search feature.</p>
                    <p>Current safe search level: <strong>{}</strong></p>
                    <p>You can <a href='/settings' style='color: #2196f3; text-decoration: underline;'>disable safe search</a> or change the filtering level in settings.</p>
                </div>
            </div>", filter_manager.get_level())
        ];
        return Ok((filtered_notice, Duration::from_millis(10)));
    }

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
    let _religious_affiliation =
        schema_builder.add_text_field("religious_affiliation", TEXT | STORED);
    let category = schema_builder.add_text_field("category", TEXT | STORED | FAST);
    let tags = schema_builder.add_text_field("tags", TEXT | STORED | FAST);
    let _privacy_track_record = schema_builder.add_text_field("ptr", TEXT | STORED);
    let _human_rights_track_record = schema_builder.add_text_field("hrr", TEXT | STORED);
    let _instant_answer = schema_builder.add_i64_field("instant_answer", STORED | FAST);
    let _language = schema_builder.add_text_field("language", TEXT | STORED);
    let schema = schema_builder.build();

    let index = if fs::read_dir(index_dir)?.count() == 0 {
        // Create the index
        let index = Index::create_in_dir("index", schema.clone())?;

        // Create a single writer for all documents
        let mut index_writer = index.writer(50_000_000)?;

        // Parse the TOML files recursively
        let mut dirs_to_process = vec![std::path::PathBuf::from("datasets")];
        let mut processed_files = 0;
        let mut error_files = Vec::new();

        while let Some(dir) = dirs_to_process.pop() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    dirs_to_process.push(path);
                    continue;
                }

                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    let path_str = path.to_string_lossy().to_string();

                    // Read the file only once
                    let data = match fs::read_to_string(&path) {
                        Ok(content) => content,
                        Err(e) => {
                            let error_msg = format!("Error reading file '{}': {}", path_str, e);
                            error_files.push(error_msg.clone());
                            eprintln!("{}", error_msg);
                            continue; // Skip this file and move to the next one
                        }
                    };

                    // Validate TOML content in one step
                    if let Err(validation_error) = validate_toml_content(&data) {
                        let error_msg = format!("Validation error in file '{}': {}", path_str, validation_error);
                        error_files.push(error_msg.clone());
                        eprintln!("{}", error_msg);
                        continue; // Skip this file and move to the next one
                    }

                    // Parse TOML only once (we know it's valid from validation)
                    let value = match toml::from_str::<Value>(&data) {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            let error_msg = format!("Error parsing TOML in file '{}': {}", path_str, e);
                            error_files.push(error_msg.clone());
                            eprintln!("{}", error_msg);
                            continue; // Skip this file and move to the next one
                        }
                    };

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
                            processed_files += 1;
                        }
                    }
                }
            }
        }

        // If there were errors, report them
        if !error_files.is_empty() {
            let error_summary = format!("Encountered {} errors while processing dataset files. First error: {}",
                error_files.len(), error_files[0]);
            eprintln!("Error summary: {}", error_summary);

            // If we have a lot of errors, suggest using the tools
            if error_files.len() > 10 {
                eprintln!("TIP: Run search with '?q=--find-invalid-files' or '?q=--inspect-dataset' to diagnose dataset issues");
            }

            // If no files were processed at all, return an error
            if processed_files == 0 {
                return Err(tantivy::TantivyError::InvalidArgument(format!(
                    "Failed to process any dataset files. Errors in {} files. First error: {}. Run search with '?q=--find-invalid-files' or '?q=--inspect-dataset' to check all files.",
                    error_files.len(), error_files[0]
                )));
            }

            println!("Processed {} files successfully despite {} errors", processed_files, error_files.len());

            // Write error report to file for later inspection
            if let Ok(mut file) = File::create("dataset_errors.log") {
                let _ = writeln!(file, "Dataset Processing Errors - {} errors found", error_files.len());
                let _ = writeln!(file, "Timestamp: {:?}", SystemTime::now());
                let _ = writeln!(file, "-----------------------------------");
                for error in &error_files {
                    let _ = writeln!(file, "{}", error);
                }
            }
        }

        // Single commit at the end
        index_writer.commit()?;
        index
    } else {
        Index::open_in_dir(index_dir)?
    };

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let start = Instant::now();

    let searcher = reader.searcher();

    // Create query parser
    let mut query_parser = QueryParser::for_index(
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

    // Configure field boosts
    query_parser.set_field_boost(title, 3.0);
    query_parser.set_field_boost(author, 2.0);
    query_parser.set_field_boost(description, 1.5);

    let words: Vec<&str> = sanitized_query.split_whitespace().collect();

    // Sort words by modifier type (include, exclude, mandatory, field)
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

    // Use cached configuration with thread-safe lazy initialization
    static CONFIG: OnceLock<Value> = OnceLock::new();
    let config = CONFIG.get_or_init(|| {
        let config_str = fs::read_to_string("Config.toml").unwrap();
        config_str.parse().unwrap()
    });

    // Get the number of results per query from Config.toml
    let results_per_query = config
        .get("engine")
        .and_then(|engine| engine.get("results_per_query"))
        .and_then(Value::as_integer)
        .unwrap_or(50) as usize;

    // Collectors
    let top_docs = searcher.search(&final_query, &TopDocs::with_limit(results_per_query))?;
    let mut scored_results = Vec::new();
    for (mut score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;

        // Check each term's matches across different fields
        let query_terms: Vec<String> = query.split_whitespace().map(|s| s.to_lowercase()).collect();

        // Track where each term matched
        let mut author_matches = 0;
        let mut content_matches = 0;

        for term in &query_terms {
            // Check author field
            if let Some(tantivy::schema::Value::Str(author_text)) = retrieved_doc.get_first(author)
            {
                if author_text.to_lowercase().contains(term) {
                    author_matches += 1;
                }
            }

            // Check content fields
            let mut found_in_content = false;
            for field in &[title, description, full_description] {
                if let Some(tantivy::schema::Value::Str(field_text)) =
                    retrieved_doc.get_first(*field)
                {
                    if field_text.to_lowercase().contains(term) {
                        found_in_content = true;
                        break;
                    }
                }
            }
            if found_in_content {
                content_matches += 1;
            }
        }

        // Apply relationship-based scoring
        if author_matches > 0 && content_matches > 0 {
            // Highest boost when terms match both author and content
            score *= 1.0 + (author_matches as f32 * 0.5) + (content_matches as f32 * 0.3);
        } else if author_matches > 0 {
            // Good boost for author matches
            score *= 1.0 + (author_matches as f32 * 0.4);
        } else if content_matches > 0 {
            // Smaller boost for content matches
            score *= 1.0 + (content_matches as f32 * 0.2);
        }

        // Get domain and category for filtering
        let mut domain = String::new();
        if let Some(tantivy::schema::Value::Str(url_text)) = retrieved_doc.get_first(url) {
            domain = extract_domain(url_text).unwrap_or_default();
        }

        let mut category_value = String::new();
        if let Some(tantivy::schema::Value::Str(cat_text)) = retrieved_doc.get_first(category) {
            category_value = cat_text.to_lowercase();
        }

        // Skip this result if domain or category should be filtered
        if filter_manager.is_enabled() {
            let should_filter_domain = !domain.is_empty() && filter_manager.should_filter_domain(&domain);
            let should_filter_category = !category_value.is_empty() && filter_manager.should_filter_category(&category_value);

            if should_filter_domain || should_filter_category {
                println!("Filtering result with domain: '{}', category: '{}' (filter domain: {}, filter category: {})",
                         domain, category_value, should_filter_domain, should_filter_category);
                continue;
            }
        }

        println!("Result passed domain/category filter check - domain: '{}', category: '{}'", domain, category_value);

        // Generate the HTML for this result
        let html = document_to_html(&retrieved_doc, &schema);
        let html_without_quotes = html.replace(", ", "");

        scored_results.push((score, html_without_quotes));
    }

    // Sort the results by score in descending order
    scored_results.sort_by(|(score1, _), (score2, _)| score2.partial_cmp(score1).unwrap());

    // Extract the html_without_quotes for each pair
    let results: Vec<String> = scored_results.into_iter().map(|(_, html)| html).collect();

    let duration = start.elapsed();
    println!("Search completed in {:?}, returned {} results", duration, results.len());
    Ok((results, duration))
}
