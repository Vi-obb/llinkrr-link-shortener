mod routes;

use std::{error::Error, iter::Product};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "link_shortener=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (prometheus_layer, metric_handle) = PrometheusLayer::pair();

    let app = Router::new()
        .route("/metrics", get(|| async move {metric_handle.render{}}))
        .route("/health", get(health));
    .layer(TraceLayer::new_for_http())
    .layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0.3000")
        .await
        .expect("Could not initialize listener");

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .expect("could not convert to local address")
    );

    axum::serve(listener, app)
        .await
        .expect("server failed to run");

    Ok(())
}
