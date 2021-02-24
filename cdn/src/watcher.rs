use notify::{watcher, RecursiveMode, Watcher};
use std::{env, path::Path, sync::mpsc::channel, time::Duration};

use crate::cache::{self, Cache};

pub fn watch(cache: Cache) {
    let (sender, receiver) = channel();

    let mut watcher =
        watcher(sender, Duration::from_secs(10)).expect("Could not start filesystem watcher.");
    let styles_dir = env::current_dir()
        .expect("Current directory path is invalid.")
        .join(Path::new("styles"));

    watcher
        .watch(
            styles_dir.to_str().unwrap_or("./styles"),
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(_) => update_cache(&cache),
            Err(error) => println!("File event error: {:?}", error),
        }
    }
}

fn update_cache(cache: &Cache) {
    match cache.try_write() {
        Ok(mut lock) => *lock = cache::compile(),
        Err(_) => {}
    }
}
