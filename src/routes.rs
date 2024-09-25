use axum::{http::StatusCode, response::IntoResponse};

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_route() {
        let app = Router::new().route("/health", get(health));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/health").await;
        response.assert_status_ok();
        response.assert_text("OK");
    }
}
