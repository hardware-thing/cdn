use gitignore;
use sass_rs::{self, Options, OutputStyle};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub type Cache = Arc<RwLock<HashMap<String, String>>>;

pub fn compile(styles_dir: String) -> HashMap<String, String> {
    let gitignore_path = PathBuf::from(styles_dir.clone())
        .canonicalize()
        .unwrap_or(PathBuf::from(format!("/sekond/{}", styles_dir)))
        .join(".gitignore");

    let pathbufs: Vec<PathBuf> = match gitignore::File::new(&gitignore_path)
        .map(|gitignore_file| gitignore_file.included_files())
    {
        // Found .gitignore and getting files succeeded
        Ok(Ok(included_files)) => included_files,
        // Something went wrong so use everything
        _ => get_files(PathBuf::from(styles_dir)),
    };

    let paths: Vec<String> = pathbufs
        .iter()
        .filter_map(|file| {
            let filename = file.display().to_string();
            // Only include stylesheet files
            if [".css", ".scss", ".sass"]
                .iter()
                .any(|ext| filename.ends_with(ext))
            {
                Some(filename)
            } else {
                None
            }
        })
        .collect();

    let mut compiled = HashMap::new();

    let include_paths: Vec<String> = paths.clone();
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
                        .replace("/", ":")
                        .to_string(),
                    css,
                );
            }
            Err(error) => println!("{}", error),
        }
    }

    compiled
}

fn get_files(folder: PathBuf) -> Vec<PathBuf> {
    let mut files = vec![];

    if let Ok(list) = fs::read_dir(folder) {
        for entry in list {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    files.append(&mut get_files(path))
                } else {
                    files.push(path)
                }
            }
        }
    }

    files
}
