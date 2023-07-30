use axum::{
    extract::{State},
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex, MutexGuard};
use axum::response::IntoResponse;

struct AppState {
    count: i32,
}

#[tokio::main]
async fn main() {
    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { count: 0 }));

    let app: Router = Router::new()
        .route("/", get(hello_world_handler))
        .with_state(shared_state);

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
    "Hello, World!"
}