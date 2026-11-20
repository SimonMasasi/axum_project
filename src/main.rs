use anyhow::Context;
use axum_project::{config::Config, db, routes, state::AppState};
use serde_json::json;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use axum::{response::Html, routing::get};
use scalar_api_reference::scalar_html_default;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "axum_project=debug,tower_http=debug,sea_orm=info".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;

    let scalar_config = json!({ "url": routes::OPENAPI_PATH });

    let db = db::connect(&config).await?;

    // The published scalar_api_reference crate doesn't ship `ui/scalar.js`, so
    // `scalar_api_reference::axum::router` 404s on `/docs/scalar.js`. Serve the
    // HTML ourselves and load the JS bundle from the jsDelivr CDN instead.
    let docs_html = scalar_html_default(&scalar_config);
    let app = routes::router(AppState { db })
        .route("/docs", get(move || async move { Html(docs_html) }));

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .with_context(|| format!("failed to bind to {}", config.addr))?;
    tracing::info!("server listening on http://{}", config.addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    tracing::info!("shutting down");
}
