#[macro_use]
extern crate rocket;

use rocket::{http::ContentType, response::Response, State};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
};

use crate::cache::Cache;

mod cache;
mod watcher;

#[get("/css?<components>")]
fn css(components: String, _cache: State<'_, Cache>) -> Response {
    let _files: Vec<String> = components
        .split("-")
        .map(|file| file.replace(":", "/").to_string())
        .collect();
    Response::build().header(ContentType::CSS).finalize()
}

#[launch]
fn rocket() -> rocket::Rocket {
    let cache: Cache = Arc::new(RwLock::new(HashMap::new()));

    let watcher_cache = cache.clone();
    thread::spawn(move || watcher::watch(watcher_cache));

    rocket::ignite().mount("/", routes![css]).manage(cache)
}
