use axum::{
    extract::{State},
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex, MutexGuard};
use axum::response::{Html, IntoResponse};

struct AppState {
    count1: i32,
    count2: i32,
}

struct AppState2 {
    message: String,
}

#[tokio::main]
async fn main() {
    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { count1: 0, count2: 10000 }));
    let shared_state2 = Arc::new(AppState2 { message: "Hello, World!".to_string() });

    let router1 = Router::new()
        .route("/", get(hello_world_handler))
        .route("/decr", get(decr_handler)).with_state(shared_state)
        ;
    let router2 = Router::new().route("/", get(message_handler))
        .with_state(shared_state2);

    let app: Router = Router::new()
        .nest("/", router1)
        .nest("/message", router2)
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
    println!("{}", state.count1);
    state.count1 = state.count1 + 1;
    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span>", state.count1))
}

async fn decr_handler(
    State(state): State<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let mut state: MutexGuard<AppState> = state.lock().unwrap();
    println!("{}", state.count2);
    state.count2 = state.count2 - 1;
    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span>", state.count2))
}

async fn message_handler(
    State(state): State<Arc<AppState2>>,
) -> impl IntoResponse {
    state.message.clone()
}