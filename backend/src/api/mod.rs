use crate::config::Config;
use std::net::SocketAddr;
use axum::{Router, routing::get};

mod calculi;

pub async fn the_fun_begins(config: Config) -> anyhow::Result<()> {

    // build our application with a single route
    let app = Router::new().route("/health", get(|| async { "OK" }));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn api_router() -> Router {
    Router::new()
        .nest("/g", calculi::router())
}