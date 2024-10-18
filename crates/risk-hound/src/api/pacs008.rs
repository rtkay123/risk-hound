use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// This is the customer
#[derive(ToSchema, Serialize)]
struct Customer {
    name: String,
}

/// expose the Customer OpenAPI to parent module
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(get_customer))
}

/// Get customer
///
/// Just return a static Customer object
#[utoipa::path(get, path = "", responses((status = OK, body = Customer)))]
async fn get_customer() -> Json<Customer> {
    Json(Customer {
        name: String::from("Bill Book"),
    })
}
