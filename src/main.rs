use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback_service(ServeDir::new("static/build"));
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}
