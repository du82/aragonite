// URL utility functions for domain extraction and homepage detection

/// Extracts domain name from a URL string without requiring url parsing
pub fn extract_domain(url: &str) -> Option<String> {
    // Remove protocol if present
    let without_protocol = url.split("://").nth(1).unwrap_or(url);

    // Handle potential query parameters and fragments
    let base_url = without_protocol.split('?').next()?;
    let base_url = base_url.split('#').next()?;

    // Get the domain part (everything before the first slash)
    let domain = base_url.split('/').next()?;

    // Remove www. prefix if present and trim any whitespace
    Some(domain.trim_start_matches("www.").trim().to_string())
}

/// Checks if URL likely points to a homepage
pub fn is_homepage(url: &str) -> bool {
    // Remove protocol if present
    let without_protocol = url.split("://").nth(1).unwrap_or(url);

    // Remove query parameters and fragments
    let base_url = without_protocol.split('?').next().unwrap_or(without_protocol);
    let base_url = base_url.split('#').next().unwrap_or(base_url);

    // Split on slashes and filter out empty segments
    let parts: Vec<&str> = base_url.split('/')
        .filter(|&s| !s.is_empty())
        .collect();

    // Check various homepage patterns
    match parts.len() {
        0 => true,  // Empty path
        1 => true,  // Just domain
        2 => {
            let last_part = parts[1].to_lowercase();
            matches!(last_part.as_str(),
                "" |
                "index.html" |
                "index.htm" |
                "index.php" |
                "index.asp" |
                "index.aspx" |
                "index.jsp" |
                "default.html" |
                "default.htm" |
                "home" |
                "main"
            )
        },
        _ => false
    }
}

/// Extracts the base domain name without TLD for matching
pub fn get_domain_name(url: &str) -> Option<String> {
    extract_domain(url).map(|domain| {
        let parts: Vec<&str> = domain.split('.').collect();
        if parts.len() >= 2 {
            // Always take the leftmost part, whether it's a subdomain or main domain
            parts[0].to_string()
        } else {
            // Single part domain
            domain
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com"), Some("example.com".to_string()));
        assert_eq!(extract_domain("http://www.example.com"), Some("example.com".to_string()));
        assert_eq!(extract_domain("www.example.com"), Some("example.com".to_string()));
        assert_eq!(extract_domain("example.com"), Some("example.com".to_string()));
        assert_eq!(extract_domain("example.com/path"), Some("example.com".to_string()));
        assert_eq!(extract_domain("example.com?query=123"), Some("example.com".to_string()));
        assert_eq!(extract_domain("example.com#fragment"), Some("example.com".to_string()));
        assert_eq!(extract_domain("example.com/path?query#fragment"), Some("example.com".to_string()));
        assert_eq!(extract_domain("  example.com  "), Some("example.com".to_string()));
    }

    #[test]
    fn test_is_homepage() {
        assert!(is_homepage("https://example.com"));
        assert!(is_homepage("https://example.com/"));
        assert!(is_homepage("https://example.com/index.html"));
        assert!(is_homepage("https://example.com/index.htm"));
        assert!(is_homepage("https://example.com/index.php"));
        assert!(is_homepage("https://example.com/index.asp"));
        assert!(is_homepage("https://example.com/index.aspx"));
        assert!(is_homepage("https://example.com/index.jsp"));
        assert!(is_homepage("https://example.com/default.html"));
        assert!(is_homepage("https://example.com/home"));
        assert!(is_homepage("https://example.com/main"));
        assert!(is_homepage("https://example.com?ref=123"));
        assert!(is_homepage("https://example.com/#section"));

        assert!(!is_homepage("https://example.com/page"));
        assert!(!is_homepage("https://example.com/path/to/page"));
        assert!(!is_homepage("https://example.com/blog/index.html"));
        assert!(!is_homepage("https://example.com/about"));
        assert!(!is_homepage("https://example.com/products/main"));
    }

    #[test]
    fn test_get_domain_name() {
        assert_eq!(get_domain_name("https://example.com"), Some("example".to_string()));
        assert_eq!(get_domain_name("https://www.example.co.uk"), Some("example".to_string()));
        assert_eq!(get_domain_name("test.example.com"), Some("test".to_string()));
        assert_eq!(get_domain_name("blog.example.co.jp"), Some("blog".to_string()));
        assert_eq!(get_domain_name("example"), Some("example".to_string()));
    }
}
