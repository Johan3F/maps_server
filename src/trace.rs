use axum::{extract::MatchedPath, http::Request, Router};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{
    fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

pub fn setup_tracing() {
    registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(layer())
        .init();
}

pub fn add_trace_layer(router: Router) -> Router {
    router.layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);
            info_span!(
                "http_request",
                method = ?request.method(),
                uri = ?request.uri(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        }),
    )
}
