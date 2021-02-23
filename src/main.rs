#[macro_use]
extern crate rocket;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
};

mod watcher;

#[get("/css")]
fn css() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket {
    let cache: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));

    let watcher_cache = cache.clone();
    thread::spawn(move || watcher::watch(watcher_cache));

    rocket::ignite().mount("/", routes![css]).manage(cache)
}
