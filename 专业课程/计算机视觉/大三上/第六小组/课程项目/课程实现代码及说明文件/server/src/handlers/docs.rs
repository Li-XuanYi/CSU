use salvo::Router;
use salvo::oapi::scalar::Scalar;

/// Scalar UI that references the generated OpenAPI spec.
pub fn scalar_router() -> Router {
    Scalar::new("/api-doc/openapi.json")
        .title("Mindsight Gallery API Reference")
        .into_router("scalar")
}
