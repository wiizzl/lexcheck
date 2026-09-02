mod handlers;
mod models;
mod router;
mod services;

#[tokio::main]
async fn main() {
    let app = router::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
