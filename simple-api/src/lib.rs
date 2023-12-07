use axum::{extract::State, routing::get, serve::Serve, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use uuid::Uuid;

pub async fn create_app(
    host: Option<&str>,
    port: u16,
    uuid: Uuid,
) -> Result<Serve<Router, Router>, String> {
    let host = host.unwrap_or("0.0.0.0");

    let app = Router::new()
        .route("/healthz", get(health_handler))
        .with_state(uuid);

    let listener = TcpListener::bind(&format!("{host}:{port}"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(axum::serve(listener, app))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    app_id: Uuid,
    message: String,
}

async fn health_handler(State(app_id): State<Uuid>) -> Json<HealthResponse> {
    Json(HealthResponse {
        app_id,
        message: "Healthy! 💓".into(),
    })
}
