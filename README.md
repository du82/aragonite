# Tor search engine
This is a research project to build a search engine designed to work wtih Onion v3 links and arbitrary key storage in the Rust programming language. It's designed with Tor's strictest settings in mind, which means no JavaScript whatsoever. Pure Rust for the backend, with HTML and CSS for the frontend.\
I've taken great care to minimize dependencies, and currently it only has three: TOML (storage), UUID (object identification), and Rocket (web server).
### Features
- Scrape and display Dread (forum) content with search parameters and the ability to follow a user's trail by typing `author=` followed by their username
- Intuitive TOML system for handling user data and search results - engine is able to deal with random key entry without crashing
- no SQL, no problems :)
- fast enough: 10k results in 15ms on localhost. No user would ever actually need that many for one single query, especially on Tor.
### Notice
This is meant to be a proof of concept research project that CAN be deployed as a hidden service on the Tor network. You'll have to supply your own search data (it goes in the `Data.toml` file), only a few examples are provided.
I've kept the crawler and tools to solve Dread's captchas proprietary and out of this repo since they'd surely fix their flaws if it was public.
