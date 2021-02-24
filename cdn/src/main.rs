#[macro_use]
extern crate rocket;

use rocket::{http::ContentType, response::Response, State};
use std::{
    env,
    io::Cursor,
    sync::{Arc, RwLock},
    thread,
};

use crate::cache::Cache;

mod cache;
mod watcher;

fn query_to_paths(components: String) -> Vec<String> {
    let mut files = vec![];
    let fragments: Vec<String> = components
        .split(",")
        .map(|file| file.replace(":", "/").to_string())
        .collect();

    for fragment in fragments {
        if fragment.contains("|") {
            if let [parent, subs] = fragment.rsplitn(2, ":").collect::<Vec<&str>>()[..] {
                for sub in subs.split("|") {
                    files.push(parent.to_string() + ":" + sub);
                }
            }
        } else {
            files.push(fragment);
        }
    }

    files
}

#[get("/css?<components>")]
fn css(components: String, cache: State<'_, Cache>) -> Response {
    let files = query_to_paths(components);
    let mut css = String::new();

    if let Ok(lock) = cache.try_read() {
        for file in files {
            css += (*lock)
                .get(&file)
                .map(|content| content.as_str())
                .unwrap_or_else(|| {
                    eprintln!("`css`: Cannot acquire `cache` RwLock; it might be poisoned.");
                    ""
                });
        }
    }

    Response::build()
        .header(ContentType::CSS)
        .sized_body(css.len(), Cursor::new(css))
        .finalize()
}

#[get("/list")]
fn list(cache: State<'_, Cache>) -> String {
    match cache.try_read() {
        Ok(lock) => {
            let mut keys = (*lock)
                .keys()
                .map(|k| k.replace("/", ":").to_owned())
                .collect::<Vec<String>>();
            keys.sort();
            keys.join("\n")
        }
        Err(_) => {
            println!("`list`: Could not acquire lock on cache");
            String::new()
        }
    }
}

#[launch]
fn rocket() -> rocket::Rocket {
    let styles_dir = env::var_os("STYLES_DIR")
        .map(|dir| dir.to_str().map(|path| path.to_owned()))
        .flatten()
        .unwrap_or("./styles".to_string());

    let cache: Cache = Arc::new(RwLock::new(cache::compile(styles_dir.clone())));

    // Spawn the file watcher to recompile on change
    let watcher_cache = cache.clone();
    thread::spawn(move || watcher::watch(styles_dir, watcher_cache));

    rocket::ignite()
        .mount("/v1", routes![css, list])
        .manage(cache)
}
