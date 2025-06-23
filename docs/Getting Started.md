# Getting Started with Aragonite<sup>α</sup>

## Introduction
Welcome to Aragonite, a privacy-focused search engine that you can self-host. Aragonite allows you to create and customize your own search index from various datasets, offering full control over what content is indexed and how it's presented to users.

## System Requirements
- **Operating System**: Linux distribution (Ubuntu, Debian, Amazon Linux, etc.) or MacOS (M1 chipset)
- **Memory**: Minimum 2GB RAM recommended, more for larger datasets
- **Storage**: Depends on the size of your datasets and index (minimum 1GB free space recommended)
- **Rust**: Latest stable or nightly version

## Installation

### 1. Clone the Repository
```bash
sudo apt install git -y
git clone https://github.com/du82/rust-login-page
cd rust-login-page
```

### 2. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup override set nightly
```

### 3. Create Required Directories
```bash
mkdir -p index
mkdir -p datasets
```

### 4. Optional: Install neofetch for System Info Feature
```bash
cd /tmp
git clone https://github.com/dylanaraps/neofetch.git
sudo cp neofetch/neofetch /usr/bin/neofetch
rm -rf neofetch
```

### 5. Build and Run for Development
```bash
cargo run
```

### 6. Or Build and Run for Production/Staging
```bash
ROCKET_ENV=staging cargo run
```

## Configuration

Aragonite has three main configuration files:

### 1. `Rocket.toml` - Web Server Configuration
This file controls the web server settings, including:
- IP address and port bindings
- Environment settings (development, staging, production)

Example configuration:
```toml
[development]
address = "127.0.0.1"
port = 12345

[staging]
address = "0.0.0.0"
port = 12345
```

### 2. `Config.toml` - Search Engine Settings
This file controls the core functionality of Aragonite:
- Branding (name, Greek letter, update date)
- Feature toggles
- Engine parameters (results per page, safe search defaults)

Example configuration:
```toml
[branding]
name = "Aragonite"              # Name of the search engine
greek = "α"                     # Greek letter to use in the logo
mirror_updated = "Apr 12, 2025" # Date of the last mirror update

[features]
neofetch_enabled = false     # Allow users to see neofetch output from the host
search_string_enabled = true # Show random strings from search_string.txt
safe_search_enabled = true   # Enable content filtering based on filters.toml

[engine]
results_per_query = 50       # Number of search results per query
safe_search_level = "moderate" # Default level: "off", "moderate", or "strict"
```

### 3. `filters.toml` - Content Filtering
This file controls safe search and content filtering:
- Filter lists for moderate and strict modes
- Category-based filtering
- Domain-based blocking

Example configuration:
```toml
[safe_search]
enabled = true
level = "moderate"

[filter_lists]
moderate = [
    "pornography",
    "explicit content",
    "adult content"
]

strict = [
    "violence",
    "gore",
    "gambling"
]

[category_filters]
enabled = false
blocked_categories = ["adult", "gambling"]

[domain_filters]
enabled = false
blocked_domains = ["adult-example.com"]
```

## Creating Datasets

Aragonite uses TOML files for data indexing. These files should be placed in the `datasets` directory.

### Dataset Structure

Each dataset is a TOML file that contains one or more documents separated by an empty line. Each document has fields like:

```toml
["https://example.com/page"]
title = "Example Page"
description = "A short description"
full_description = "A longer, more detailed description of the page content"
author = "Author Name"
year_published = 2025
clearnet = "https://example.com/page"
onion = "http://exampleonionaddress.onion"  # Optional
category = "Technology"
tags = ["example", "documentation", "web"]
language = "en"
ptr = "good"  # Privacy track record: good, bad, neutral
hrr = "good"  # Human rights record: good, bad, neutral
```

### Dataset Tips

1. Organize datasets by domain or category for easier management
2. Use subdirectories in `datasets/` to organize related TOML files
3. Include as much metadata as possible for better search results
4. Keep documents separated by empty lines
5. Use Pascal case for dataset filenames (e.g., `Technology.toml`)

## Running Searches

Once you've configured Aragonite and added datasets:

1. Start the server with `cargo run` or `ROCKET_ENV=staging cargo run`
2. Open a browser and navigate to the URL shown in the console (default: http://127.0.0.1:12345)
3. Enter a search query in the search box
4. The first search will be slow as it builds the index
5. Subsequent searches will be much faster

## Search Features

Aragonite supports several advanced search features:

- **Inclusive search**: `search term` (default)
- **Exclusive search**: `-term` (exclude results with this term)
- **Mandatory search**: `+term` (results must include this term)
- **Field-specific search**: `field:value` (e.g., `author:smith` or `category:technology`)
- **Random search**: Click the "Random" button for a random search query

## Reindexing Data

If you modify your datasets, you need to clear the index to rebuild it. There are two ways to do this:

### Method 1: Using the reindex flag

You can use the built-in reindex flag to automatically clear the index directory and rebuild it:

```bash
cargo run -i
```

The `-i` flag will delete all contents of the index folder and trigger reindexing when the server starts.

### Method 2: Manual reindexing

Alternatively, you can manually clear the index and restart the server:

```bash
rm -rf index/*
cargo run
```

Both methods will rebuild the index from the updated datasets.

## Troubleshooting

### Common Issues

1. **Empty search results**: Ensure your datasets are properly formatted and the index directory exists
2. **Slow initial search**: This is normal - Aragonite is building the index for the first time
3. **Missing dependencies**: Make sure Rust nightly is installed and all dependencies are resolved
4. **Permission errors**: Ensure you have proper permissions for the directories

### Getting Help

If you encounter issues not covered here, please check:

- The `docs/hosting` directory for platform-specific guides
- The `docs/development` directory for development information
- Open an issue on the GitHub repository

## Next Steps

- Explore advanced configuration options in `Config.toml`
- Customize your instance's appearance using themes
- Set up a production deployment with proper security settings
- Contribute to the project by adding new features or datasets

Happy searching with Aragonite<sup>α</sup>!