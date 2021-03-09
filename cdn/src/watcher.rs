use console::style;
use notify::{watcher, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};

use crate::cache::{self, Cache};

pub fn watch(styles_dir: String, cache: Cache) {
    let (sender, receiver) = channel();

    let mut watcher =
        watcher(sender, Duration::from_secs(10)).expect("Could not start filesystem watcher.");
    watcher
        .watch(styles_dir.clone(), RecursiveMode::Recursive)
        .expect("Could not start filesystem watcher.");

    loop {
        match receiver.recv() {
            Ok(_) => update_cache(styles_dir.clone(), &cache),
            Err(error) => println!(
                "{} {}",
                style("File event error:").red().bold(),
                style(error).red()
            ),
        }
    }
}

fn update_cache(styles_dir: String, cache: &Cache) {
    match cache.try_write() {
        Ok(mut lock) => *lock = cache::compile(styles_dir.clone()),
        Err(_) => {}
    }
}
