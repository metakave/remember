use axum::{
    routing::{get, put},
    Router, Json, extract::{Path, State},
};
use sqlx::{SqlitePool, Row};
use sqlx::migrate::MigrateDatabase;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use shared::{VisionPayload, GoalsPayload, Reminder};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    let db_url = "sqlite:goals.db";
    
    // Create database if not exists
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap_or(false) {
        sqlx::Sqlite::create_database(db_url).await.unwrap();
    }
    
    let pool = SqlitePool::connect(db_url).await.unwrap();
    
    // Run schema migrations/setup
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS vision (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            vision TEXT NOT NULL DEFAULT '',
            mission TEXT NOT NULL DEFAULT ''
        );"
    ).execute(&pool).await.unwrap();
    
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS goals (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            yearly TEXT NOT NULL DEFAULT '',
            quarterly TEXT NOT NULL DEFAULT '',
            monthly TEXT NOT NULL DEFAULT '',
            weekly TEXT NOT NULL DEFAULT ''
        );"
    ).execute(&pool).await.unwrap();
    
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reminders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            is_completed BOOLEAN NOT NULL DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );"
    ).execute(&pool).await.unwrap();
    
    // Seed default rows
    sqlx::query("INSERT OR IGNORE INTO vision (id, vision, mission) VALUES (1, '', '');")
        .execute(&pool)
        .await
        .unwrap();
        
    sqlx::query("INSERT OR IGNORE INTO goals (id, yearly, quarterly, monthly, weekly) VALUES (1, '', '', '', '');")
        .execute(&pool)
        .await
        .unwrap();

    let state = AppState { pool };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/api/vision", get(get_vision).put(update_vision))
        .route("/api/goals", get(get_goals).put(update_goals))
        .route("/api/reminders", get(get_reminders).post(create_reminder))
        .route("/api/reminders/:id", put(toggle_reminder).delete(delete_reminder))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend API running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_vision(State(state): State<AppState>) -> Json<VisionPayload> {
    let row = sqlx::query("SELECT vision, mission FROM vision WHERE id = 1")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    Json(VisionPayload {
        vision: row.get("vision"),
        mission: row.get("mission"),
    })
}

async fn update_vision(
    State(state): State<AppState>,
    Json(payload): Json<VisionPayload>,
) -> Json<VisionPayload> {
    sqlx::query("UPDATE vision SET vision = ?, mission = ? WHERE id = 1")
        .bind(&payload.vision)
        .bind(&payload.mission)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(payload)
}

async fn get_goals(State(state): State<AppState>) -> Json<GoalsPayload> {
    let row = sqlx::query("SELECT yearly, quarterly, monthly, weekly FROM goals WHERE id = 1")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    Json(GoalsPayload {
        yearly: row.get("yearly"),
        quarterly: row.get("quarterly"),
        monthly: row.get("monthly"),
        weekly: row.get("weekly"),
    })
}

async fn update_goals(
    State(state): State<AppState>,
    Json(payload): Json<GoalsPayload>,
) -> Json<GoalsPayload> {
    sqlx::query("UPDATE goals SET yearly = ?, quarterly = ?, monthly = ?, weekly = ? WHERE id = 1")
        .bind(&payload.yearly)
        .bind(&payload.quarterly)
        .bind(&payload.monthly)
        .bind(&payload.weekly)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(payload)
}

async fn get_reminders(State(state): State<AppState>) -> Json<Vec<Reminder>> {
    let rows = sqlx::query("SELECT id, text, is_completed FROM reminders ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    let reminders = rows
        .into_iter()
        .map(|r| Reminder {
            id: Some(r.get("id")),
            text: r.get("text"),
            is_completed: r.get::<bool, _>("is_completed"),
            created_at: None,
        })
        .collect();
    Json(reminders)
}

async fn create_reminder(
    State(state): State<AppState>,
    Json(payload): Json<Reminder>,
) -> Json<Reminder> {
    let text = payload.text;
    let is_completed = payload.is_completed;
    
    let res = sqlx::query("INSERT INTO reminders (text, is_completed) VALUES (?, ?)")
        .bind(&text)
        .bind(is_completed)
        .execute(&state.pool)
        .await
        .unwrap();
        
    let id = res.last_insert_rowid();
    
    Json(Reminder {
        id: Some(id),
        text,
        is_completed,
        created_at: None,
    })
}

async fn toggle_reminder(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<Reminder>,
) -> Json<Reminder> {
    sqlx::query("UPDATE reminders SET is_completed = ? WHERE id = ?")
        .bind(payload.is_completed)
        .bind(id)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(payload)
}

async fn delete_reminder(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Json<bool> {
    sqlx::query("DELETE FROM reminders WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(true)
}
