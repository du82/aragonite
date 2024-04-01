# Welcome to Aragonite<sup>α</sup> documentation

1. Read through the `/docs/hosting` directory to find your platform and build instructions.
2. Configure the `Rocket.toml` web server config, specifically IP and Port are important.
3. Configure the `Config.toml` file to change how the search engine and UI work. Updates to this file take effect immediately.
4. Create files ending in `.toml` inside the `datasets` directory and copy-paste the toml example into it. Change the data as you see fit. The engine will automatically detect any files ending in `.toml` and add them to the index. Add TOML tables seperated by an empty line for best results.
5. Create the `index` directory in the root of the project if you haven't already.