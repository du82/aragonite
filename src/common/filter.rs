// filter.rs
// Provides filtering functionality for search results

use tantivy::query::{Query, TermQuery, Occur};
use tantivy::schema::{Schema, Field};
use tantivy::Term;
use tantivy::schema::IndexRecordOption;

/// Represents different types of filters that can be applied to search results
pub enum FilterType {
    Obscure,
    Recent,
    Popular,
    // Add more filter types as needed
    None,
}

impl FilterType {
    /// Convert a string representation to the corresponding FilterType
    pub fn from_str(filter_str: &str) -> Self {
        match filter_str.to_lowercase().as_str() {
            "obscure" => FilterType::Obscure,
            "recent" => FilterType::Recent,
            "popular" => FilterType::Popular,
            // Add more mappings as needed
            _ => FilterType::None,
        }
    }
}

/// Apply the specified filter to the query clauses
pub fn apply_filter(
    filter_type: &FilterType,
    schema: &Schema,
    query_clauses: &mut Vec<(Occur, Box<dyn Query>)>
) -> Result<(), tantivy::TantivyError> {
    match filter_type {
        FilterType::Obscure => apply_obscure_filter(schema, query_clauses),
        FilterType::Recent => apply_recent_filter(schema, query_clauses),
        FilterType::Popular => apply_popular_filter(schema, query_clauses),
        // Add more filter implementations as needed
        FilterType::None => Ok(()), // Do nothing for no filter
    }
}

/// Apply filter for obscure content
fn apply_obscure_filter(
    schema: &Schema,
    query_clauses: &mut Vec<(Occur, Box<dyn Query>)>
) -> Result<(), tantivy::TantivyError> {
    // Prioritize content with "obscure" in categories or tags

    if let Ok(category_field) = schema.get_field("category") {
        let obscure_term = Term::from_field_text(category_field, "obscure");
        let obscure_query = TermQuery::new(obscure_term, IndexRecordOption::Basic);
        query_clauses.push((Occur::Should, Box::new(obscure_query)));
    }

    if let Ok(tags_field) = schema.get_field("tags") {
        let obscure_tag_term = Term::from_field_text(tags_field, "obscure");
        let obscure_tag_query = TermQuery::new(obscure_tag_term, IndexRecordOption::Basic);
        query_clauses.push((Occur::Should, Box::new(obscure_tag_query)));
    }

    // Deprioritize content with high visibility
    if let Ok(instant_answer_field) = schema.get_field("instant_answer") {
        let term = Term::from_field_i64(instant_answer_field, 1);
        let query = TermQuery::new(term, IndexRecordOption::Basic);
        query_clauses.push((Occur::MustNot, Box::new(query)));
    }

    Ok(())
}

fn apply_recent_filter(
    schema: &Schema,
    query_clauses: &mut Vec<(Occur, Box<dyn Query>)>
) -> Result<(), tantivy::TantivyError> {
    // Try to use year fields if they exist and are indexed properly
    let try_year_updated = schema.get_field("year_updated");
    let try_year_published = schema.get_field("year_published");

    if try_year_updated.is_ok() || try_year_published.is_ok() {
        // Use whichever field is available
        let current_year = 2025 as i64;

        if let Ok(year_field) = try_year_updated.or(try_year_published) {
            // Use this field for filtering
            let this_year_term = Term::from_field_i64(year_field, current_year);
            let this_year_query = TermQuery::new(this_year_term, IndexRecordOption::Basic);
            query_clauses.push((Occur::Should, Box::new(this_year_query)));

            // Also boost last year's results
            let last_year_term = Term::from_field_i64(year_field, current_year - 1);
            let last_year_query = TermQuery::new(last_year_term, IndexRecordOption::Basic);
            query_clauses.push((Occur::Should, Box::new(last_year_query)));
        }
    }

    // Always return Ok - this way the filter won't cause errors
    // even if the fields aren't properly indexed
    Ok(())
}

/// Apply filter for popular content
fn apply_popular_filter(
    schema: &Schema,
    query_clauses: &mut Vec<(Occur, Box<dyn Query>)>
) -> Result<(), tantivy::TantivyError> {
    // Prioritize content marked for instant answers

    if let Ok(instant_answer_field) = schema.get_field("instant_answer") {
        let term = Term::from_field_i64(instant_answer_field, 1);
        let query = TermQuery::new(term, IndexRecordOption::Basic);
        query_clauses.push((Occur::Should, Box::new(query)));
    }

    Ok(())
}
