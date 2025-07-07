// Convert Tantivy documents to HTML for display

use std::collections::HashMap;
use tantivy::schema::Schema;
use tantivy::schema::Value;
use tantivy::Document;

pub fn document_to_html(doc: &Document, schema: &Schema) -> String {
    let mut fields = HashMap::new();
    for field_value in doc.field_values() {
        let field_name = schema.get_field_name(field_value.field());
        if let Value::Str(value) = field_value.value() {
            let value_without_newlines = value.replace("\n", "").replace("[", "");
            fields.insert(field_name, value_without_newlines);
        }
    }

    let default_value = "".to_string();
    let title = fields
        .get("title")
        .unwrap_or_else(|| &&default_value)
        .replace("[", "")
        .split(',')
        .collect::<Vec<&str>>()
        .join("");
    let title = if title.len() > 200 {
        format!("{}...", &title.chars().take(100).collect::<String>())
    } else {
        title
    };
    let description = fields
        .get("description")
        .unwrap_or_else(|| &&default_value)
        .replace("[", "")
        .split(',')
        .collect::<Vec<&str>>()
        .join("");
    let description = if description.len() > 200 {
        format!("{}...", &description.chars().take(200).collect::<String>())
    } else {
        description
    };
    let author = fields
        .get("author")
        .unwrap_or_else(|| &&default_value)
        .replace("[", "")
        .split(',')
        .collect::<Vec<&str>>()
        .join("");
    let author_html = if !author.is_empty() {
        format!(
            "<a class='pill' href='/search?q={}'>author: {}</a>",
            author, author
        )
    } else {
        "".to_string()
    };
    let tags = fields.get("tags").unwrap_or_else(|| &&default_value);
    let tags_html = if !tags.is_empty() {
        format!(
            "<a class='pill' href='/search?q={}'>{}</a>",
            tags.to_lowercase(),
            tags.to_lowercase()
        )
    } else {
        "".to_string()
    };
    let category = fields
        .get("category")
        .unwrap_or_else(|| &&default_value)
        .replace(", ", "");
    let category_html = if !category.is_empty() {
        format!(
            "<a class='pill' href='/search?q=category:{}'>{}</a>",
            category.to_lowercase(),
            category.to_lowercase()
        )
    } else {
        "".to_string()
    };
    let privacy_track_record = fields
        .get("ptr")
        .unwrap_or_else(|| &&default_value)
        .replace(", ", "");
    let ptr_html = if !privacy_track_record.is_empty() {
        format!(
            "<a title='Privacy track record' class='pill' href='/search?q=ptr:{}'>PTR: {}</a>",
            privacy_track_record.to_lowercase(),
            privacy_track_record.to_lowercase()
        )
    } else {
        "".to_string()
    };
    let human_rights_track_record = fields
        .get("hrr")
        .unwrap_or_else(|| &&default_value)
        .replace(", ", "");
    let hrr_html = if !human_rights_track_record.is_empty() {
        format!(
            "<a title='Human rights track record' class='pill' href='/search?q=hrr:{}'>HRR: {}</a>",
            human_rights_track_record.to_lowercase(),
            human_rights_track_record.to_lowercase()
        )
    } else {
        "".to_string()
    };
    let language = fields
        .get("language")
        .unwrap_or_else(|| &&default_value)
        .replace(", ", "");
    let language_html = if !language.is_empty() {
        format!(
            "<a class='pill' href='/search?q=language:{}'>language: {}</a>",
            language.to_lowercase(),
            language.to_lowercase()
        )
    } else {
        "".to_string()
    };
    let url = fields
        .get("clearnet")
        .unwrap_or_else(|| &&default_value)
        .replace("[", "")
        .split(',')
        .collect::<Vec<&str>>()
        .join("");
    let onion = fields
        .get("onion")
        .unwrap_or_else(|| &&default_value)
        .replace("[", "")
        .split(',')
        .collect::<Vec<&str>>()
        .join("");

    let url_html = match (onion.is_empty(), url.is_empty()) {
        (false, false) => format!("<p class='search-result-url-row'><a class='search-result-url' href='{}'>Onion</a> | {}</p>", onion, url),
        (false, true) => format!("<p class='search-result-url-row'><a class='search-result-url' href='{}'>Onion</a></p>", onion),
        (true, false) => format!("<p class='search-result-url-row'>{}</p>", url),
        (true, true) => "".to_string(),
    };

    // Instant Answer
    let mut instant_answer_sent = false;
    let instant_answer = if !instant_answer_sent {
        instant_answer_sent = true;
        fields
            .get("instant_answer")
            .unwrap_or_else(|| &&default_value)
            .replace("[", "")
            .split(',')
            .collect::<Vec<&str>>()
            .join("")
    } else {
        "".to_string()
    };

    let result_html = if instant_answer == "sports" {
        format!("<div class='instant-answer'><details><summary>Instant Answer - never sponsored</summary><div class='search-result'><h3><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></h3><p class='sports-description'>{}</p><div class='sports-related-row'>{}{}{}{}{}{}</div></div></details></div>",
        url, title, description, ptr_html, hrr_html, author_html, category_html, tags_html, language_html)
    } else if instant_answer == "linux" {
        format!("<div class='instant-answer'><details><summary>System Info</summary><h1>{}</h1><p class='search-result-description'>{}</p></details></div>",
        title, description)
    } else {
        format!("<div class='search-result'>{}<h3><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></h3><p class='search-result-description'>{}</p><div class='search-result-related-row'>{}{}{}{}{}{}</div></div>",
        url_html, url, title, description, ptr_html, hrr_html, author_html, category_html, tags_html, language_html)
    };

    result_html
}
