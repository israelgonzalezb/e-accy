// Import the actix_web crate.
use actix_web::{server, App, HttpRequest, Responder, fs, Path };
// Import the std::env crate.
use std::env;

fn respond(req: &HttpRequest) -> Result<fs::NamedFile, server::HttpHandlerTask10> {
    let path = req.path(); // .unwrap_or("index")
    // let path = Path::new(&path);
    // let path = path.to_str().unwrap();
    let file_requested = format!("static/{}.html", path); // Fix this
    fs::NamedFile::open(file_requested) // This is risky too
}

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().expect("PORT must be a number");

    server::new(|| {
        App::new()
            .resource("/*", |r| r.f(respond))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run();
}
