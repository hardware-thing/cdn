use notify::{watcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    env,
    path::Path,
    sync::{mpsc::channel, Arc, RwLock},
    time::Duration,
};

pub fn watch(_cache: Arc<RwLock<HashMap<String, String>>>) {
    let (sender, receiver) = channel();

    let mut watcher =
        watcher(sender, Duration::from_secs(10)).expect("Could not start filesystem watcher.");
    let styles_dir = env::current_dir()
        .expect("Current directory path is invalid.")
        .join(Path::new("styles"));

    watcher
        .watch(
            styles_dir.to_str().unwrap_or("./styled"),
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(_) => println!("biuldin"),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
