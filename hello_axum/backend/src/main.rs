use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use axum::body::{Body, boxed};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tower::util::ServiceExt;

#[derive(Clone)]
struct AppState {
}

impl Default for AppState {
    fn default() -> Self {
        AppState {}
    }
}

type SharedAppState = State<Arc<RwLock<AppState>>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Counter {
    value: i32
}

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(AppState::default()));
    let app = Router::new()
        .route(
            "/api/plant",
            get(get_handler)
                .put(put_handler)
        )
        .fallback_service(get(|req| async move {
            match ServeDir::new(String::from("dist")).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .with_state(state);

    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9090);

    println!("listening on http://{}", socket_address);

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn get_handler() -> Response {
    (StatusCode::OK, Json(Counter { value: 42 })).into_response()
}

async fn put_handler() -> Response {
    StatusCode::OK.into_response()
}
