use axum::serve;
use axum::{routing::get, Router};
use once_cell::sync::Lazy;
use rand::random;
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::net::TcpListener;

// Global static variable to store the random number
static RANDOM_NUMBER: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(random::<u8>()));

async fn hello_world() -> String {
    // Lock the mutex to read the precomputed random number
    let num = *RANDOM_NUMBER.lock().unwrap();
    format!("Hello, World! {}", num)
}

#[tokio::main]
async fn main() {
    // Create a router with a single route
    let app = Router::new().route("/", get(hello_world));

    // Set the address for the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("Listening on http://{}", addr);
    println!(
        "Generated Random Number: {}",
        *RANDOM_NUMBER.lock().unwrap()
    );

    // Bind to a TCP listener instead of using Server::bind()
    let listener = TcpListener::bind(addr).await.unwrap();

    // Start the server using `axum::serve()`
    serve(listener, app.into_make_service()).await.unwrap();
}
