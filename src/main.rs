use actix_web::{server, App, HttpRequest, Responder, fs, Error, HttpResponse, Path, Result, ResponseError};
use std::env;

fn respond(path: Path<String>) -> Result<fs::NamedFile> {
    let file_requested = format!("static/{}", path.into_inner());
    fs::NamedFile::open(file_requested).map_err(|_| ResponseError::from(HttpResponse::NotFound().body("File not found").into()))
}

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().expect("PORT must be a number");

    server::new(|| {
        App::new()
            .resource("/{name:.*\\.html}", |r| r.f(respond)) // Serve only files with .html extension
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run();
}
