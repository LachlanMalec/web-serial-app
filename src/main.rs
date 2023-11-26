use axum::{response::Html, routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(serve_html));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_html() -> Html<&'static str> {
    Html("<!DOCTYPE html><html><body><h1>Hello, world!</h1></body></html>")
}
