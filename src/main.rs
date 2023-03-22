use axum::{routing::get, Router};
use std::net::SocketAddr;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!(
        "Starting {name} version {version} with profile {profile}",
        name = crate::built_info::PKG_NAME,
        version = crate::built_info::PKG_VERSION,
        profile = crate::built_info::PROFILE
    );
    tracing::info!(
        "Built {date} from git commit {commit}",
        date = crate::built_info::BUILT_TIME_UTC,
        commit = crate::built_info::GIT_VERSION.unwrap_or("None")
    );

    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn root() -> &'static str {
    tracing::info!("Coin Coin");
    "Hello, World!"
}