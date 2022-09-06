// Import the actix_web crate.
use actix_web::{server, App, HttpRequest, Responder, fs};
// Import the std::env crate.
use std::env;

// Create a function that will return the index.html file.
fn index(_req: &HttpRequest) -> impl Responder {
    // Open the index.html file.
    // The index.html file should be placed in the same directory as the Rust file.
    fs::NamedFile::open("static/index.html").unwrap()
}

fn main() {
    // Get the port number from the environment variable.
    let port = env::var("PORT")
        // If the environment variable is not set, use port 3000.
        .unwrap_or_else(|_| "3000".to_string())
        // Parse the port number as a number.
        .parse()
        // If the port number is not a number, panic.
        .expect("PORT must be a number");

    // Create a new server.
    server::new(|| {
        // Create a new application.
        App::new()
            // Create a new resource.
            .resource("/", |r| r.f(index))
                // Create a new route.
    })
                    // Use the index function to return the index.html file.
    .bind(("0.0.0.0", port))
                // End the route.
    .expect("Can not bind to port 8000")
            // End the resource.
    .run();
        // End the application.
}
    // End the server.
    // Bind the server to the port.
    // If the server can not bind to the port, panic.
    // Run the server.
