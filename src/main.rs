use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a number");

    HttpServer::new(|| App::new().service(fs::Files::new("/", "./static").index_file("index.html").show_files_listing()))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
