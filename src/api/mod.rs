use axum::Router;
use axum::{
    extract::{MatchedPath, Request},
    response::Response,
};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};
use tracing::{Span, info, info_span};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Check health
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Healthy", body = String),
    )
)]
async fn get_health_handler() -> &'static str {
    "healthy"
}

#[derive(utoipa::OpenApi)]
#[openapi(
    info(title = env!("CARGO_PKG_NAME"),),
)]
struct PublicApiDoc;

pub fn build_router() -> Router<()> {
    let health_handler_route = OpenApiRouter::new().routes(routes![get_health_handler]);

    // Combine routers in one OpenApiRouter
    let (router, _) = OpenApiRouter::with_openapi(PublicApiDoc::openapi())
        .merge(health_handler_route)
        .split_for_parts();

    router
}

pub async fn run_server(address: String, cancellation: CancellationToken) -> eyre::Result<()> {
    info!("Starting API server on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(
        listener,
        build_router()
            .layer(CorsLayer::permissive())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                    )
                    })
                    .on_request(|request: &Request<_>, span: &Span| {
                        tracing::info!(parent: span, "Incoming request: {} {}", request.method(), request.uri());
                    })
                    .on_response(|response: &Response, latency: Duration, span: &Span| {
                        tracing::info!(parent: span, status = ?response.status(), latency = ?latency, "Response sent");
                    })
                    .on_failure(
                        |error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                            tracing::error!(parent: span, error = ?error, latency = ?latency, "Request failed");
                        },
                    ),
            ),
    ).with_graceful_shutdown(cancellation.cancelled_owned()).await?;

    Ok(())
}
