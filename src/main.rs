// Import the actix_web crate.
use actix_web::{server, App, HttpRequest, Responder, fs};
// Import the std::env crate.
use std::env;

// Create a function that will return a requested file from the /static dir
fn respond(_req: &HttpRequest) -> impl Responder {
    
    // Get the /{name} file, otherwise get index
    let to = req.match_info().get("name").unwrap_or("index");
    
    // Open the corresponding html file if it exists, otherwise serve null.html
    fs::NamedFile::open(format!("static/{}.html", to)).unwrap_or("static/null.html");
}

fn main() {
    // Get the port number from the environment variable.
    let port = env::var("PORT")
        // If the environment variable is not set, use port 3000.
        .unwrap_or_else(|_| "3000".to_string()) // Has something to do with monads lol
        // Parse the port number as a number.
        .parse()
        // If the port number is not a number, panic.
        .expect("PORT must be a number");

    // Create a new server.
    server::new(|| {
        // Create a new application.
        App::new()
            // Create a new resource and route
            // Use the respond function to return a file
            .resource("/", |r| r.f(respond))
    })
     // Bind the server to the port.
    .bind(("0.0.0.0", port))
     // If the server can not bind to the port, panic.
    .expect("Can not bind to port 8000")
    // Run that sth homie.
    .run();
}
