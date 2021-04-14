# Sekond 🕶️

## 🗂️ Directory structure

- `cdn` — the actual CDN in Rust
- `styles` — folder where all the style files are
- `builder` — a tiny web tool for creating the URLs (🚧 WIP)

## 🚀 Running it

Run `cargo r` to get it up and running. To disable the builder frontend build set `NO_BUILDER=1`.

For production use, we recommend using Docker. (🚧 WIP)

## 🔗 The URL

There are only two endpoints:

- `/v1/css` — the endpoint for your `<link>` tag
- `/v1/list` — get the list of supported styles on your instance

You can select what styles you want to fetch with a URL query loosely inspired by Google Fonts API:

`/v1/css?folder:subfolder:component`

To fetch multiple components from one folder without repeating yourself you can do:

`/v1/css?button:primary|secondary|disabled`
