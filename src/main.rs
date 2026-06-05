mod db;

use std::sync::{Arc, Mutex};

use serde::Deserialize;
use axum::{
    Router,
    routing,
    extract::{State, Path},
    http::StatusCode,
    response::{Html, Json},
};
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let Ok(db_conn) = Connection::open("./rust.sqlite") else { panic!("failed to open db conn.") };

    db_conn.execute("PRAGMA journal_mode = WAL;", []).ok();
    db_conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id       INTEGER PRIMARY KEY,
            done     INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1)),
            task     TEXT    NOT NULL,
            priority INTEGER NOT NULL DEFAULT 0
        );",
        (),
    ).ok();

    let app = Router::new()
                     .route("/", routing::get(root))
                     .route("/list", routing::get(list))
                     .route("/task", routing::post(create))
                     .route("/task/{id}/task", routing::put(update_task))
                     .route("/task/{id}/priority/{priority}", routing::post(update_priority))
                     .route("/task/{id}/check", routing::post(check))
                     .route("/task/{id}/uncheck", routing::post(uncheck))
                     .route("/task/{id}/delete", routing::post(delete))
                     .with_state(Arc::new(Mutex::new(db_conn)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(std::include_str!("./index.html"))
}

async fn list(State(db_conn): State<Arc<Mutex<Connection>>>) -> Json<Vec<db::Task>> {
    Json(db::Task::query_all(&db_conn.lock().unwrap()).unwrap())
}

#[derive(Debug, Deserialize)]
struct TaskCreate {
    task: String,
}

async fn create(
    State(db_conn): State<Arc<Mutex<Connection>>>,
    Json(task): Json<TaskCreate>,
) -> Json<db::Task> {
    Json(db::Task::create(&db_conn.lock().unwrap(), &task.task).unwrap())
}

async fn update_task(
    Path(id): Path<i32>,
    State(db_conn): State<Arc<Mutex<Connection>>>,
    Json(task): Json<TaskCreate>,
) -> StatusCode {
    if let Ok(_) = db::Task::update_task(&db_conn.lock().unwrap(), &id, &task.task) {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Deserialize)]
struct TaskUpdatePriority {
    id: i32,
    priority: i32,
}

async fn update_priority(
    Path(task_update_priority_params): Path<TaskUpdatePriority>,
    State(db_conn): State<Arc<Mutex<Connection>>>,
) -> StatusCode {
    if let Ok(_) = db::Task::update_priority(
                       &db_conn.lock().unwrap(),
                       &task_update_priority_params.id,
                       &task_update_priority_params.priority
                   ) {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn check(
    Path(id): Path<i32>,
    State(db_conn): State<Arc<Mutex<Connection>>>,
) -> StatusCode {
    if let Ok(_) = db::Task::update_done(&db_conn.lock().unwrap(), &id, &1) {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn uncheck(
    Path(id): Path<i32>,
    State(db_conn): State<Arc<Mutex<Connection>>>,
) -> StatusCode {
    if let Ok(_) = db::Task::update_done(&db_conn.lock().unwrap(), &id, &0) {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn delete(
    Path(id): Path<i32>,
    State(db_conn): State<Arc<Mutex<Connection>>>,
) -> StatusCode {
    if let Ok(_) = db::Task::delete(&db_conn.lock().unwrap(), &id) {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
