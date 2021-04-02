use rocket::{
    fairing::AdHoc,
    http::{
        hyper::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL},
        ContentType,
    },
    Response, State,
};
use rocket_contrib::serve;
use std::io::Cursor;

use crate::cache::Cache;

fn query_to_paths(components: String) -> Vec<String> {
    let mut files = vec![];
    let fragments: Vec<String> = components.split(",").map(|file| file.to_string()).collect();

    for fragment in fragments {
        match fragment.rsplitn(2, ":").collect::<Vec<&str>>()[..] {
            // Pipe branching at lower levels
            [subs, parent] => {
                for sub in subs.split("|") {
                    files.push(parent.to_string() + ":" + sub);
                }
            }
            // Top-level pipe branching and normal filenames
            [path] => {
                for file in path.split("|") {
                    files.push(file.to_string());
                }
            }
            // Garbage in, garbage out
            _ => {}
        }
    }

    files
}

#[get("/css?<components>")]
fn css(components: String, cache: State<'_, Cache>) -> Response {
    let files = query_to_paths(components);

    if let Ok(lock) = cache.try_read() {
        let mut css = String::new();

        for file in files {
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

pub fn serve(cache: Cache) -> rocket::Rocket {
    println!("Serving the styles…");
    rocket::ignite()
        .mount("/v1", routes![css, list])
        .mount("/", serve::StaticFiles::from("./builder/dist/"))
        .manage(cache)
        .attach(AdHoc::on_response("Caching headers", |_, res| {
            Box::pin(async move {
                res.set_raw_header(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*");
                res.set_raw_header(CACHE_CONTROL.as_str(), "private; max-age=86400");
                res.set_raw_header("timing-allow-origin", "*");
            })
        }))
}

mod tests {
    #[test]
    fn test_query_to_paths() {
        assert_eq!(
            super::query_to_paths("button:primary".to_string()),
            vec!["button:primary"]
        );
        assert_eq!(
            super::query_to_paths("button:primary|secondary".to_string()),
            vec!["button:primary", "button:secondary"]
        );
    }
}
