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

struct SharedStates {
    state1: Arc<Mutex<AppState>>,
    state2: Arc<Mutex<AppState2>>,
}

impl Clone for SharedStates {
    fn clone(&self) -> Self {
        Self {
            state1: Arc::clone(&self.state1),
            state2: Arc::clone(&self.state2),
        }
    }
}
#[tokio::main]
async fn main() {
    let shared_states = SharedStates {
        state1: Arc::new(Mutex::new(AppState { count1: 0, count2: 10000 })),
        state2: Arc::new(Mutex::new(AppState2 { message: "Hello!".to_string() })),
    };

    let router1 = Router::new()
        .route("/", get(hello_world_handler))
        .route("/decr", get(decr_handler));
    let router2 = Router::new().route("/", get(message_handler));

    let app: Router = Router::new()
        .nest("/", router1)
        .nest("/message", router2)
        .with_state(shared_states)
        ;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world_handler(
    State(shared_states): State<SharedStates>,
) -> impl IntoResponse {
    let mut state1: MutexGuard<AppState> = shared_states.state1.lock().unwrap();
    println!("{}", state1.count1);
    state1.count1 = state1.count1 + 1;

    let mut state2: MutexGuard<AppState2> = shared_states.state2.lock().unwrap();
    state2.message = "last access: incr".into();
    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span><br /><span>{}</span>", state1.count1, state2.message))
}

async fn decr_handler(
    State(shared_states): State<SharedStates>,
) -> impl IntoResponse {
    let mut state1: MutexGuard<AppState> = shared_states.state1.lock().unwrap();
    println!("{}", state1.count2);
    state1.count2 = state1.count2 - 1;

    let mut state2: MutexGuard<AppState2> = shared_states.state2.lock().unwrap();
    state2.message = "last access: decr".into();

    Html(format!("<a href=\"/decr\">decr</a><br /><a href=\"/\">incr</a><br /><span>current: {}</span><br /><span>{}</span>", state1.count2, state2.message))
}

async fn message_handler(
    State(shared_states): State<SharedStates>,
) -> impl IntoResponse {
    let mut state2 = shared_states.state2.lock().unwrap();
    state2.message.clone()
}