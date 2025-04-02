use std::net::SocketAddr;

use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, patch, post},
    Router,
};
use hyper::StatusCode;
use outro_08::ticket_store::TicketStore;
use outro_08::{
    ticket::{Ticket, TicketId, TicketParseError, TicketPatch},
    ticket_store::PatchError,
};
use serde_json::{json, Value};
use thiserror::Error;
#[derive(Debug, Error)]
struct AppError(#[source] anyhow::Error);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_msg = self.0.to_string();
        let status = StatusCode::BAD_REQUEST;
        let body = Json(json!({"error": error_msg}));
        (status, body).into_response()
    }
}

impl From<PatchError> for AppError {
    fn from(err: PatchError) -> Self {
        AppError(anyhow::anyhow!(err))
    }
}

#[axum::debug_handler]
async fn create_ticket(
    State(store): State<TicketStore>,
    Json(ticket): Json<Ticket>,
) -> Result<Json<Value>, AppError> {
    let tid = ticket.id;
    store.insert(ticket);
    Ok(Json(json!({"id": tid})))
}
#[axum::debug_handler]
async fn get_ticket(
    State(store): State<TicketStore>,
    Json(id): Json<TicketId>,
) -> Result<Json<Ticket>, AppError> {
    let ticket = store.get(id);
    match ticket {
        Some(ticket) => Ok(Json(ticket)),
        None => Err(AppError(anyhow::anyhow!("Ticket not found"))),
    }
}
#[axum::debug_handler]
async fn update_ticket(
    State(store): State<TicketStore>,
    Json(patch): Json<TicketPatch>,
) -> Result<(), AppError> {
    store.patch(patch.id, patch)?;
    Ok(())
}

fn get_app() -> Router {
    let store = TicketStore::new();
    let app = Router::new()
        .route(
            "/ticket",
            post(create_ticket).get(get_ticket).patch(update_ticket),
        )
        .with_state(store);
    app
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/ticket",
        post(create_ticket).get(get_ticket).patch(update_ticket),
    );
    let store = TicketStore::new();
    let app = app.with_state(store);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_server() -> String {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind TCP listener");
        let used_addr = listener.local_addr().unwrap();
        let app = get_app();
        tokio::spawn(async move {
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        });
        format!("http://{}", used_addr)
    }
    #[tokio::test]
    async fn test_create_get_ticket() {
        let server_url = setup_server().await;
        let client = reqwest::Client::new();
        let ticket = Ticket::new(
            42.into(),
            "this is a title".try_into().unwrap(),
            "this is a description".try_into().unwrap(),
            "todo".try_into().unwrap(),
        );
        let response = client
            .post(format!("{}/ticket", server_url))
            .json(&ticket)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let get_response = client
            .get(format!("{}/ticket", server_url))
            .json(&ticket.id)
            .send()
            .await
            .unwrap();
        assert_eq!(get_response.status(), StatusCode::OK);
        let fetched_ticket: Ticket = get_response.json().await.unwrap();
        assert_eq!(fetched_ticket.clone(), ticket);
        eprintln!("{:?}", fetched_ticket);
    }
    #[tokio::test]
    async fn test_patch_ticket(){
        let server_url = setup_server().await;
        let client = reqwest::Client::new();
        let ticket = Ticket::new(
            42.into(),
            "this is a title".try_into().unwrap(),
            "this is a description".try_into().unwrap(),
            "todo".try_into().unwrap(),
        );
        let response = client
            .post(format!("{}/ticket", server_url))
            .json(&ticket)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let patch = TicketPatch::new(ticket.id, None, None, Some("done".try_into().unwrap()));
        let patch_response = client
            .patch(format!("{}/ticket", server_url))
            .json(&patch)
            .send()
            .await
            .unwrap();
        assert_eq!(patch_response.status(), StatusCode::OK);
        let get_response = client
            .get(format!("{}/ticket", server_url))
            .json(&ticket.id)
            .send()
            .await
            .unwrap();
        assert_eq!(get_response.status(), StatusCode::OK);
        let fetched_ticket: Ticket = get_response.json().await.unwrap();
        assert_eq!(fetched_ticket.status, "done".try_into().unwrap());
        eprintln!("patch test output:\n{:?}", fetched_ticket);
    }
}
