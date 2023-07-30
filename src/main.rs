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
    let shared_state2 = Arc::new(Mutex::new(AppState2 { message: "Hello, World!".to_string() }));

    let router1 = Router::new()
        .route("/", get(hello_world_handler))
        .route("/decr", get(decr_handler))
        .with_state(shared_state)
        .with_state(shared_state2.clone())
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
    State(state): State<Arc<Mutex<AppState>>>,
    State(state2): State<Arc<Mutex<AppState2>>>,
) -> impl IntoResponse {
    let mut state: MutexGuard<AppState> = state.lock().unwrap();
    println!("{}", state.count1);
    state.count1 = state.count1 + 1;

    let mut state2: MutexGuard<AppState2> = state2.lock().unwrap();
    state2.message = "last access: incr".into();
    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span><br /><span>{}</span>", state.count1, state2.message))
}

async fn decr_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    State(state2): State<Arc<Mutex<AppState2>>>,
) -> impl IntoResponse {
    let mut state: MutexGuard<AppState> = state.lock().unwrap();
    println!("{}", state.count2);
    state.count2 = state.count2 - 1;

    let mut state2: MutexGuard<AppState2> = state2.lock().unwrap();
    state2.message = "last access: decr".into();

    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span><br /><span>{}</span>", state.count2, state2.message))
}

async fn message_handler(
    State(state): State<Arc<Mutex<AppState2>>>,
) -> impl IntoResponse {
    let state: MutexGuard<AppState2> = state.lock().unwrap();
    state.message.clone()
}