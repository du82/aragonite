use std::collections::HashMap;
use tantivy::schema::Schema;
use tantivy::Document;
use tantivy::schema::Value;

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
            "<a class='inline-block px-3 py-1 rounded-full bg-gray-800/50 hover:bg-gray-700 text-sm text-gray-300 transition-colors' href='/search?q={}'>author: {}</a>",
            author, author
        )
    } else {
        "".to_string()
    };
    let tags = fields.get("tags").unwrap_or_else(|| &&default_value);
    let tags_html = if !tags.is_empty() {
        format!(
            "<a class='inline-block px-3 py-1 rounded-full bg-gray-800/50 hover:bg-gray-700 text-sm text-gray-300 transition-colors' href='/search?q={}'>{}</a>",
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
            "<a class='inline-block px-3 py-1 rounded-full bg-gray-800/50 hover:bg-gray-700 text-sm text-gray-300 transition-colors' href='/search?q=category:{}'>{}</a>",
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
            "<a title='Privacy track record' class='inline-block px-3 py-1 rounded-full bg-gray-800/50 hover:bg-gray-700 text-sm text-gray-300 transition-colors' href='/search?q=ptr:{}'>PTR: {}</a>",
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
            "<a title='Human rights track record' class='inline-block px-3 py-1 rounded-full bg-gray-800/50 hover:bg-gray-700 text-sm text-gray-300 transition-colors' href='/search?q=hrr:{}'>HRR: {}</a>",
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
            "<a class='inline-block px-3 py-1 rounded-full bg-gray-800/50 hover:bg-gray-700 text-sm text-gray-300 transition-colors' href='/search?q=language:{}'>language: {}</a>",
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
        (false, false) => format!(
            "<p class='text-sm text-gray-400 mb-2'><a class='hover:text-purple-400 transition-colors' href='{}'>Onion</a> | {}</p>",
            onion, url
        ),
        (false, true) => format!(
            "<p class='text-sm text-gray-400 mb-2'><a class='hover:text-purple-400 transition-colors' href='{}'>Onion</a></p>",
            onion
        ),
        (true, false) => format!("<p class='text-sm text-gray-400 mb-2'>{}</p>", url),
        (true, true) => "".to_string(),
    };

    let instant_answer = fields
        .get("instant_answer")
        .unwrap_or_else(|| &&default_value)
        .replace("[", "")
        .split(',')
        .collect::<Vec<&str>>()
        .join("");

    let result_html = if instant_answer == "sports" {
        format!(
            "<div class='bg-gray-800/50 w-[576px] backdrop-blur rounded-lg p-4 mb-4'><details><summary class='text-lg text-purple-400 cursor-pointer mb-2'>Instant Answer - never sponsored</summary><div><h3><a href='{}' target='_blank' rel='noopener noreferrer' class='text-xl font-semibold hover:text-purple-400 transition-colors'>{}</a></h3><p class='text-gray-300 my-2'>{}</p><div class='flex flex-wrap gap-2'>{}{}{}{}{}{}</div></div></details></div>",
            url, title, description, ptr_html, hrr_html, author_html, category_html, tags_html, language_html
        )
    } else if instant_answer == "linux" {
        format!(
            "<div class='bg-gray-800/50 w-[576px] backdrop-blur rounded-lg p-4 mb-4'><details><summary class='text-lg text-purple-400 cursor-pointer mb-2'>System Info</summary><h1 class='text-xl font-semibold'>{}</h1><p class='text-gray-300 mt-2'>{}</p></details></div>",
            title, description
        )
    } else {
        format!(
            "<div class='bg-gray-800/50 w-[576px] backdrop-blur rounded-lg p-4 mb-4 hover:border hover:border-purple-500/50 transition-all'>{}<h3><a href='{}' target='_blank' rel='noopener noreferrer' class='text-xl font-semibold hover:text-purple-400 transition-colors'>{}</a></h3><p class='text-gray-300 my-2'>{}</p><div class='flex flex-wrap gap-2'>{}{}{}{}{}{}</div></div>",
            url_html, url, title, description, ptr_html, hrr_html, author_html, category_html, tags_html, language_html
        )
    };

    result_html
}
