// ============================================
// DEMO 2: Web Server — REST API mit axum
// ============================================
// Zeigt: async/await, typsicheres JSON, shared State
// Echte Projekte: Discord (axum), Cloudflare Workers, npm Registry
//
// Cargo.toml:
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1", features = ["derive"] }
//
// Ausführen: cargo run
// Testen:
//   curl http://localhost:3000/users
//   curl -X POST http://localhost:3000/users \
//     -H "Content-Type: application/json" \
//     -d '{"name":"Alice","age":30}'

use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id:   u32,
    name: String,
    age:  u32,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age:  u32,
}

// Thread-sicherer shared State — Arc<Mutex<T>>
type SharedState = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(vec![
        User { id: 1, name: "Alice".into(), age: 30 },
        User { id: 2, name: "Bob".into(),   age: 25 },
    ]));

    let app = Router::new()
        .route("/users",     get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server läuft auf http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_users(
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> Json<Vec<User>> {
    Json(state.lock().unwrap().clone())
}

async fn get_user_by_id(
    axum::extract::State(state): axum::extract::State<SharedState>,
    Path(id): Path<u32>,
) -> Result<Json<User>, StatusCode> {
    state.lock().unwrap()
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    axum::extract::State(state): axum::extract::State<SharedState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = state.lock().unwrap();
    let user = User {
        id:   users.len() as u32 + 1,
        name: payload.name,
        age:  payload.age,
    };
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}
