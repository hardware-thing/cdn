#[macro_use]
extern crate rocket;

use console::style;
use log::error;
use pretty_env_logger;
use std::{
    env, process,
    sync::{Arc, RwLock},
    thread,
};

use crate::cache::Cache;

mod cache;
mod endpoint;
mod watcher;

#[launch]
fn rocket() -> rocket::Rocket {
    pretty_env_logger::init();

    // Get the styles directory from ENV
    // This variable is used in all `fs` manipulations
    let styles_dir = env::var("STYLES_DIR").unwrap_or("./styles".to_string());

    let cache: Cache = Arc::new(RwLock::new(cache::compile(styles_dir.clone())));

    // Spawn the file watcher to recompile on change
    let watcher_cache = cache.clone();
    thread::spawn(move || watcher::watch(styles_dir, watcher_cache));

    // Compile the URL builder frontend
    if env::var("NO_BUILDER")
        .map(|wants_builder| wants_builder.as_str() != "1")
        .unwrap_or(true)
    {
        if !process::Command::new("yarn")
            .arg("build")
            .current_dir("./builder")
            .status()
            .unwrap_or_else(|_| {
                error!(
                    "Could not run {}. Is {} installed?",
                    style("yarn build").bold(),
                    style("yarn").bold()
                );
                process::exit(1)
            })
            .success()
        {
            error!("Compiling the URL builder frontend failed");
            process::exit(1)
        }
    };

    // Take it to the moon!
    endpoint::serve(cache)
}
