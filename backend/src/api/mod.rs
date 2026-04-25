use crate::{api, config::Config};
use std::{net::SocketAddr, sync::Arc};
use axum::{Extension, Router, routing::get};
use sqlx::PgPool;
use tower::ServiceBuilder;

mod calculi;
mod user;
mod types;

pub use types::error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct AppContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn the_fun_begins(config: Config, db: PgPool) -> anyhow::Result<()> {

    // build our application with a single route
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(AppContext{
                config: Arc::new(config),
                db
            })
        )
    );

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn api_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/calculi", calculi::router())
        .nest("/user", user::router())
}