use actix_cors::Cors;

/// Building CORS with right... CORS :)
#[inline]
pub fn build_cors(allowed_origins: &str) -> Cors {
    let mut cors = Cors::default().allow_any_header().allow_any_method();

    // Emulating the wild card behavior
    if allowed_origins == "*" {
        return cors.allow_any_origin();
    }
    for origin in allowed_origins.split(',') {
        cors = cors.allowed_origin(origin)
    }

    cors
}
