use axum::{
    extract::{State},
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex, MutexGuard};
use axum::response::{Html, IntoResponse};

struct AppState {
    count: i32,
}

#[tokio::main]
async fn main() {
    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { count: 0 }));
    let shared_state_sub: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { count: 10000 }));

    let app: Router = Router::new()
        .route("/", get(hello_world_handler))
        .with_state(shared_state)
        .route("/decr", get(decr_handler))
        .with_state(shared_state_sub)
        ;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world_handler(
    State(state): State<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let mut state: MutexGuard<AppState> = state.lock().unwrap();
    println!("{}", state.count);
    state.count = state.count + 1;
    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span>", state.count))
}
async fn decr_handler(
    State(state): State<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let mut state: MutexGuard<AppState> = state.lock().unwrap();
    println!("{}", state.count);
    state.count = state.count - 1;
    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span>", state.count))
}
