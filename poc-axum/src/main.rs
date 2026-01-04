use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    run_hello_world().await;

    //run_basic_crud().await;
    println!("Hello, world!");
}

async fn run_hello_world() {
    async fn hello_world() -> String {
        "Hello, World2!".to_string()
    }

    async fn mirror_body_string(body: String) -> String {
        let concat = format!("body: {}", body);
        concat
    }

    async fn health_check() -> impl IntoResponse {
        Json(json!({ "status": "ok", "message": "Server is running!" }))
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(hello_world))
        .route("/mirror", get(mirror_body_string))
        .route("/health", get(health_check));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn run_basic_crud() {
    // TODO how this load the properties?
    dotenvy::dotenv().expect("Failed to initialize dotenvy.");
    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let router = Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/{task_id}", patch(update_task).delete(delete_task))
        .with_state(db_pool);

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to address");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .expect("Failed to run server");

    println!("Hello, world!");
}

async fn create_task(
    // TODO how works State? idem Json
    State(db_pool): State<PgPool>,
    Json(task): Json<CreateTaskReq>,
) -> Result<(StatusCode), (StatusCode, String)> {
    dbg!(&task);

    // when the table task is not db, this line throws error: "error: error returned from database: relation "tasks" does not exist"
    // throws that in compile time WHY?
    // row is type unknown, TODO why?
    let row = sqlx::query_as!(
        CreateTaskRow,
        "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
        task.name,
        task.priority
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok(StatusCode::CREATED)
}

// TODO State(pg_pool) learn more
async fn get_tasks(
    State(pg_pool): State<PgPool>,
) -> Result<Json<Vec<TaskRow>>, (StatusCode, String)> {
    let rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    // TODO return json itself not a string
    Ok(Json(rows))
}

async fn update_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(task): Json<UpdateTaskReq>,
) -> Result<(), (StatusCode, String)> {
    let mut query = "UPDATE tasks SET task_id = $1".to_owned();
    let mut i = 2;
    if task.name.is_some() {
        query.push_str(&format!(", name = ${i}"));
        i = i + 1;
    }
    if task.priority.is_some() {
        query.push_str(&format!(", priority = ${i}"));
        i = i + 1;
    }

    query.push_str(&format!(" WHERE task_id = $1"));

    let mut s = sqlx::query(&query).bind(task_id);
    if task.name.is_some() {
        s = s.bind(task.name);
    }

    if task.priority.is_some() {
        s = s.bind(task.priority);
    }

    s.execute(&db_pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok(())
}

async fn delete_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> Result<(StatusCode), (StatusCode, String)> {
    sqlx::query!("DELETE FROM tasks WHERE task_id = $1", task_id)
        .execute(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
struct TaskRow {
    task_id: i32,
    name: String,
    priority: Option<i32>,
}

#[derive(Deserialize, Debug)]
struct CreateTaskReq {
    name: String,
    priority: Option<i32>,
}

#[derive(Serialize)]
struct CreateTaskRow {
    task_id: i32,
}

#[derive(Deserialize)]
struct UpdateTaskReq {
    name: Option<String>,
    priority: Option<i32>,
}
