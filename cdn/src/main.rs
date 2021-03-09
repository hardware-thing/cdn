#[macro_use]
extern crate rocket;

use console::style;
use rocket::{fairing::AdHoc, http::ContentType, response::Response, State};
use rocket_contrib::serve;
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
    let fragments: Vec<String> = components.split(",").map(|file| file.to_string()).collect();

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

    if let Ok(lock) = cache.try_read() {
        let mut css = String::new();
        println!("{:?}", *lock);
        for file in files {
            println!("{}", file);
            css += (*lock).get(file.as_str()).unwrap_or(&"".to_string());
        }

        Response::build()
            .header(ContentType::CSS)
            .sized_body(css.len(), Cursor::new(css))
            .finalize()
    } else {
        eprintln!("`css`: Cannot acquire `cache` RwLock; it might be poisoned.");
        Response::build().finalize()
    }
}

#[get("/list")]
fn list(cache: State<'_, Cache>) -> String {
    match cache.try_read() {
        Ok(lock) => {
            let mut keys = (*lock)
                .keys()
                .map(|k| k.to_owned())
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
    // Pretty panics
    std::panic::set_hook(Box::new(|info| {
        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            println!("{} {}", style("Error:").red().bold(), style(payload).red());
        } else {
            if let Some(location) = info.location() {
                println!(
                    "{} {}:{}:{}",
                    style("Error occured at").red().bold(),
                    location.file(),
                    location.line(),
                    location.column()
                );
            } else {
                println!(
                    "{}{}",
                    style("Error occured ").red().bold(),
                    style("but I do not know where…").red()
                );
            }
        }
    }));

    // Get the styles directory from ENV
    // This variable is used in all `fs` manipulations
    let styles_dir = env::var_os("STYLES_DIR")
        .map(|dir| dir.to_str().map(|path| path.to_owned()))
        .flatten()
        .unwrap_or("./styles".to_string());

    let cache: Cache = Arc::new(RwLock::new(cache::compile(styles_dir.clone())));

    // Spawn the file watcher to recompile on change
    let watcher_cache = cache.clone();
    thread::spawn(move || watcher::watch(styles_dir, watcher_cache));

    // Take it to the moon!
    rocket::ignite()
        .mount("/v1", routes![css, list])
        .mount("/", serve::StaticFiles::from("./builder/dist/"))
        .manage(cache)
        .attach(AdHoc::on_response("Caching headers", |_, res| Box::pin(async move {
            res.set_header(ContentType::HTML);
        })))
}
