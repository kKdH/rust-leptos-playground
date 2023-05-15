use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use axum::body::{Body, boxed};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tower::util::ServiceExt;

#[derive(Clone)]
struct AppState {
    counter: i32
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            counter: 0,
        }
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
                .post(post_handler)
                .delete(delete_handler)
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

async fn get_handler(State(state): SharedAppState) -> Response {
    (StatusCode::OK, Json(Counter { value: state.read().unwrap().counter })).into_response()
}

async fn post_handler(State(state): SharedAppState) -> Response {
    state.write().unwrap().counter += 1;
    StatusCode::OK.into_response()
}

async fn delete_handler(State(state): SharedAppState) -> Response {
    state.write().unwrap().counter = 0;
    StatusCode::OK.into_response()
}
