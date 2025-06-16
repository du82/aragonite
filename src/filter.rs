// filters.rs

use std::collections::HashSet;
use std::fs;
use toml::Value;

pub struct FilterManager {
    enabled: bool,
    level: String,
    filter_terms: HashSet<String>,
    blocked_domains: HashSet<String>,
    blocked_categories: HashSet<String>,
}

impl FilterManager {
    pub fn new() -> Self {
        let mut filter_manager = FilterManager {
            enabled: true,
            level: "moderate".to_string(),
            filter_terms: HashSet::new(),
            blocked_domains: HashSet::new(),
            blocked_categories: HashSet::new(),
        };

        filter_manager.load_config();
        filter_manager.load_filters();
        filter_manager
    }

    fn load_config(&mut self) {
        match fs::read_to_string("Config.toml") {
            Ok(config_str) => {
                if let Ok(config) = config_str.parse::<Value>() {
                    // Get safe search enabled status
                    if let Some(features) = config.get("features") {
                        if let Some(safe_search_enabled) = features.get("safe_search_enabled").and_then(Value::as_bool) {
                            self.enabled = safe_search_enabled;
                        }
                    }

                    // Get safe search level
                    if let Some(engine) = config.get("engine") {
                        if let Some(safe_search_level) = engine.get("safe_search_level").and_then(Value::as_str) {
                            self.level = safe_search_level.to_string();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading Config.toml: {}", e);
            }
        }
    }

    fn load_filters(&mut self) {
        println!("Loading filters for level: {}", self.level);
        match fs::read_to_string("filters.toml") {
            Ok(filters_str) => {
                if let Ok(filters) = filters_str.parse::<Value>() {
                    // Load filter terms based on current level
                    if let Some(filter_lists) = filters.get("filter_lists") {
                        // Always load moderate terms if level is moderate or strict
                        if self.level == "moderate" || self.level == "strict" {
                            if let Some(moderate_terms) = filter_lists.get("moderate").and_then(Value::as_array) {
                                println!("Loading {} moderate filter terms", moderate_terms.len());
                                for term in moderate_terms {
                                    if let Some(term_str) = term.as_str() {
                                        self.filter_terms.insert(term_str.to_lowercase());
                                        println!("Added moderate filter term: '{}'", term_str.to_lowercase());
                                    }
                                }
                            }
                        }

                        // Add strict terms if level is strict
                        if self.level == "strict" {
                            if let Some(strict_terms) = filter_lists.get("strict").and_then(Value::as_array) {
                                println!("Loading {} strict filter terms", strict_terms.len());
                                for term in strict_terms {
                                    if let Some(term_str) = term.as_str() {
                                        self.filter_terms.insert(term_str.to_lowercase());
                                        println!("Added strict filter term: '{}'", term_str.to_lowercase());
                                    }
                                }
                            }
                        }
                    }

                    // Load domain filters if enabled
                    if let Some(domain_filters) = filters.get("domain_filters") {
                        if let Some(enabled) = domain_filters.get("enabled").and_then(Value::as_bool) {
                            if enabled {
                                if let Some(domains) = domain_filters.get("blocked_domains").and_then(Value::as_array) {
                                    for domain in domains {
                                        if let Some(domain_str) = domain.as_str() {
                                            self.blocked_domains.insert(domain_str.to_lowercase());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Load category filters if enabled
                    if let Some(category_filters) = filters.get("category_filters") {
                        if let Some(enabled) = category_filters.get("enabled").and_then(Value::as_bool) {
                            if enabled {
                                if let Some(categories) = category_filters.get("blocked_categories").and_then(Value::as_array) {
                                    for category in categories {
                                        if let Some(category_str) = category.as_str() {
                                            self.blocked_categories.insert(category_str.to_lowercase());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading filters.toml: {}", e);
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_level(&self) -> &str {
        &self.level
    }

    pub fn should_filter_query(&self, query: &str) -> bool {
        println!("Checking if query should be filtered: '{}', enabled: {}, level: {}", query, self.enabled, self.level);
        
        if !self.enabled || self.level == "off" {
            println!("Filtering disabled or level off, not filtering");
            return false;
        }

        let query_lower = query.to_lowercase();
        
        // Check if query contains any filtered terms
        println!("Number of filter terms loaded: {}", self.filter_terms.len());
        for term in &self.filter_terms {
            if query_lower.contains(term) {
                println!("Query contains filtered term: '{}', filtering", term);
                return true;
            }
        }

        println!("No filtered terms found in query, not filtering");
        false
    }

    pub fn should_filter_domain(&self, domain: &str) -> bool {
        if !self.enabled || self.level == "off" {
            return false;
        }

        let domain_lower = domain.to_lowercase();
        self.blocked_domains.contains(&domain_lower)
    }

    pub fn should_filter_category(&self, category: &str) -> bool {
        if !self.enabled || self.level == "off" {
            return false;
        }

        let category_lower = category.to_lowercase();
        self.blocked_categories.contains(&category_lower)
    }

    pub fn set_level(&mut self, level: &str) {
        println!("Setting safe search level to: {}", level);
        self.level = level.to_string();
        // Reload filters after changing the level
        self.filter_terms.clear();
        self.load_filters();
        println!("After reload: {} filter terms loaded", self.filter_terms.len());
    }

    pub fn enable(&mut self, enabled: bool) {
        println!("Setting safe search enabled to: {}", enabled);
        self.enabled = enabled;
    }
}
