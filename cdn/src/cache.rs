use sass_rs::{self, Options, OutputStyle};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub type Cache = Arc<RwLock<HashMap<String, String>>>;

pub fn compile(styles_dir: String) -> HashMap<String, String> {
    let mut compiled = HashMap::new();

    let paths: Vec<String> = get_files(PathBuf::from(styles_dir))
        .iter()
        .filter_map(|path| path.to_str().map(|path| path.to_string()))
        .collect();

    let include_paths = paths.clone();
    for path in paths {
        match sass_rs::compile_file(
            path.as_str(),
            Options {
                output_style: OutputStyle::Compressed,
                precision: 3,
                indented_syntax: false,
                include_paths: include_paths.clone(),
            },
        ) {
            Ok(css) => {
                compiled.insert(
                    path.trim_start_matches("./styles/")
                        .trim_end_matches(".scss")
                        .to_string(),
                    css,
                );
            }
            Err(error) => println!("{}", error),
        }
    }

    println!("{:?}", compiled);

    compiled
}

fn get_files(folder: PathBuf) -> Vec<PathBuf> {
    let mut files = vec![];

    if let Ok(list) = fs::read_dir(folder) {
        for entry in list {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    files.append(&mut get_files(entry.path()))
                } else {
                    files.push(entry.path())
                }
            }
        }
    }

    files
}
