use console::style;
use log::{error, info};
use rocket::{
    fairing::AdHoc,
    http::{
        hyper::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL},
        ContentType, Status,
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
        error!(
            "'{}': Cannot acquire {} RwLock; it might be poisoned.",
            style("css").bold(),
            style("cache").bold()
        );
        Response::build()
            .status(Status::InternalServerError)
            .finalize()
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
            error!(
                "'{}': Could not acquire lock on cache",
                style("list").bold()
            );
            String::new()
        }
    }
}

pub fn serve(cache: Cache, builder: bool) -> rocket::Rocket {
    info!("Serving the styles…");
    let mut server = rocket::ignite()
        .mount("/v1", routes![css, list])
        .manage(cache)
        .attach(AdHoc::on_response("Caching headers", |_, res| {
            Box::pin(async move {
                res.set_raw_header(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*");
                res.set_raw_header(CACHE_CONTROL.as_str(), "private; max-age=86400");
                res.set_raw_header("timing-allow-origin", "*");
            })
        }));

    if builder {
        server = server.mount("/", serve::StaticFiles::from("./builder/dist/"));
    }

    server
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
