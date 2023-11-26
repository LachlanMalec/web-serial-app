use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Form, Router, Server,
};
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_html))
        .route("/write", post(write_data));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_html() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn write_data(Form(input): Form<Input>) -> StatusCode {
    let mut file = File::create("data.txt").await.unwrap();
    file.write_all(input.data.as_bytes()).await.unwrap();
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct Input {
    data: String,
}
