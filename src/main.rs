


use std::time::Duration;
use axum::{
    extract::{Path,State},
    routing::{get,patch},
    http::{StatusCode},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use sqlx::{
    Pool, 
    MySql, 
    Error, 
    MySqlPool,
    };
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    // build our application with a single route
    let server_address = std::env::var("SERVER_ADDR").unwrap_or("localhost:3333".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let db_pool = MySqlPoolOptions::new()
    .max_connections(64)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&database_url)
    .await
    .expect("can't connect to database");


    // Create a socket address (IPv6 binding)
    let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create tcp listener");
    
   



    let app = Router::new()
        .route("/", get(root))
        .route("/second", get(second_route))
        .route("/tasks", get(get_tasks))
        .with_state(db_pool);
    // Run the app with hyper, listening on the specified address
    axum::serve(listener, app)
    .await
    .expect("Failed to start server");

}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello World, from Axum!"
}


async fn second_route() -> &'static str {
    "This is the second route!"
}

#[derive(Serialize)]
struct TaskRow {
  task_id: i32,
  name: String,
  priority: Option<i32>,
}

#[derive(Deserialize)]
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

async fn get_tasks(State(db_pool):State<MySqlPool>)
->Result<(StatusCode,String),(StatusCode,String)>
{
    let rows = sqlx::query_as!(TaskRow,"SELECT * FROM axumtest1.tasks ORDER BY task_id")
        .fetch_all(&db_pool).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    return Ok((
            StatusCode::OK,
            json!({"success":true, "data":rows}).to_string()
    ));

}



