# Building Aragonite
1. Switch to Rust development version
2. Create a new directory in the project root (same level that `Cargo.toml` is on) named `index` if it doesn't exist. Make sure this directory is empty prior to running.
3. Create a new directory in the project root if it doesn't exist, and enter your data in TOML table format. See example below. Leave one space between each entry.
4. Type `cargo run`
5. Open the URL that it gives you in the browser, and try searching for something that you entered into your datasets. It'll be slow for the first query, this is normal as it's indexing the data.
6. Wait for it to load, you should see results in the browser. The `index` folder that you created should have files in it now. Every time you change something in datasets, you must delete the contents of the `datasets` folder (but not the folder itself) to make your changes appear.

## Things to note
- Your datasets must be in this path: `/datasets/Example.toml` where `Example.toml` is the name of your dataset. Do not use special characters or spaces in the dataset names, this slows down performance. Pascal case is recommended.
- You can name the datasets whatever you want. It's recommended to name them with categories of content that you wish to index. You might have one called `Movies.toml` which includes data about movies, alongside one called `Sports.toml` which includes data about sports.
