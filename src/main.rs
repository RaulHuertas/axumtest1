


use std::time::Duration;
use axum::{
    extract::{Path,State}, http::{HeaderMap, Method, StatusCode}, 
    routing::{get,patch,post,put,options}, 
    Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use sqlx::{
    PgPool,
    Pool, 
    Error, 
    MySqlPool,
    };
use sqlx::postgres::PgPoolOptions;
use axum::response::{Response};



pub mod dbmodel;
use crate::dbmodel::devices::TestStr;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let mut test_str = TestStr {
        name: Some("Test".to_string()),
        priority: Some(1),
    };
    println!("test_str: {:?}", test_str);
    test_str.increase_priority();
    println!("test_str after increase: {:?}", test_str);
    // build our application with a single route
    let server_address = std::env::var("SERVER_ADDR").unwrap_or("localhost:3333".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let db_pool = PgPoolOptions::new()
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
        .route("/extractorTest1", get(ext_test1))
        .route("/extractorTest2", post(ext_test2))
        .route("/test_handler", post(test_handler))        
        .route("/full_output_control", get(full_output_control))
        .with_state(db_pool);
    // Run the app with hyper, listening on the specified address
    axum::serve(listener, app)
    .await
    .expect("Failed to start server");

}

//async fn full_output_control()->(StatusCode, HeaderMap,String) {
async fn full_output_control()->Response<String> {
    let response = Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .body(json!({"success": true, "message": "Full output control successful"}).to_string());
    response.unwrap()
}


enum TestCommands{
    None,
    Start,
    Stop,
    Pause,
    Resume,
}
impl TestCommands {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "start" => TestCommands::Start,
            "stop" => TestCommands::Stop,
            "pause" => TestCommands::Pause,
            "resume" => TestCommands::Resume,
            _ => TestCommands::None,
        }
    }
}

#[derive(Deserialize,Debug,Serialize)]
struct TestMessage {
  name: Option<String>,
  command: Option<String>,
}

async fn test_handler(
    Json(payload): Json<TestMessage>,
) -> (StatusCode, String) {
    println!("Received payload: {:?}", payload);
    if let Some(name) = payload.name {
        println!("Name: {}", name);
    } else {
        println!("No name provided");
    }
    let mut command_received = TestCommands::None;
    if let Some(command) = payload.command {
        println!("Command: {}", command);
        command_received = TestCommands::from_str(&command); 
    } else {
        println!("No command provided");
    }
    match command_received {
        TestCommands::Start => println!("Starting..."),
        TestCommands::Stop => println!("Stopping..."),
        TestCommands::Pause => println!("Pausing..."),
        TestCommands::Resume => println!("Resuming..."),
        TestCommands::None => println!("No valid command provided"),
    }

    (
        StatusCode::OK,
        json!({"success": true, "message": "Test handler successful"}).to_string(),
    )
}

async fn ext_test1(headers: HeaderMap)-> (StatusCode, String) {
    println!("Headers: {:?}", headers);
    if let Some(game_is_running)= headers.get("GameRunning") {
        println!("Game is running header present: {}", game_is_running.to_str().unwrap_or("0"));
    }else{
        println!("Game is running header not present");
    }     

    (
        StatusCode::OK,
        json!({"success": true, "message": "Extractor test successful"}).to_string(),
    )   
}

async fn ext_test2(method:Method)-> (StatusCode, String) {
    println!("Method: {:?}", method);
    (
        StatusCode::OK,
        json!({"success": true, "message": "Extractor test 2 successful"}).to_string(),
    )
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
  name: Option<String>,
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

async fn get_tasks(State(db_pool):State<PgPool>)
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

async fn update_task(
  State(db_pool): State<PgPool>,
  Path(task_id): Path<i32>,
  Json(task): Json<UpdateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let mut query = "UPDATE tasks SET task_id = $1".to_owned();

  let mut i = 2;

  if task.name.is_some() {
    query.push_str(&format!(", name = ${i}"));
    i = i + 1;
  };

  if task.priority.is_some() {
    query.push_str(&format!(", priority = ${i}"));
  };

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

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}


