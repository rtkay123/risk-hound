pub mod pacs008;

use std::net::{Ipv6Addr, SocketAddr};

use anyhow::Result;
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

const CUSTOMER_TAG: &str = "customer";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CUSTOMER_TAG, description = "Customer API endpoints"),
    )
)]
struct ApiDoc;

pub async fn router(port: u16) -> Result<()> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .nest("/api/customer", pacs008::router())
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}

/// Get health of the API.
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health() -> &'static str {
    "ok"
}
