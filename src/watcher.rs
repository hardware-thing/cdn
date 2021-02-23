use notify::{watcher, RecursiveMode, Watcher};
use std::{env, path::Path, sync::mpsc::channel, time::Duration};

use crate::cache::Cache;

pub fn watch(_cache: Cache) {
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
            Ok(_) => println!("biuldin"),
            Err(error) => println!("File event error: {:?}", error),
        }
    }
}
