
use axum::body::Body;
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{DateTime,NaiveDateTime};

use axum::{
    extract::{Path,State}, http::{HeaderMap, Method, StatusCode}, 
    routing::{get,patch,post,put,options}, 
    Json, Router
};

use sqlx::{
    PgPool,
    Pool, 
    Error, 
    Postgres,
};

#[derive(Deserialize,Debug,Serialize)]
pub struct TestStr {
  pub name: Option<String>,
  pub priority: Option<i32>,
}

impl TestStr {
    pub fn increase_priority(&mut self) {
        if let Some(priority) = &mut self.priority {
            *priority += 1;
        }
    }
}

#[derive(Deserialize,Debug,Serialize)]
pub struct Version{
    pub mayor: i32,
    pub medium: i32,
    pub minor: i32,
}

impl Version {

    pub fn new(mayor: i32, medium: i32, minor: i32) -> Self {
        Version { mayor, medium, minor }
    }
    
    pub fn from_int(version: i32) -> Self {
        let mayor = version / 1000_000;
        let medium = (version / 1000) % 1000;
        let minor = version % 1000;
        Version { mayor, medium, minor }
    }

    pub fn to_int(&self) -> i32 {
        return self.mayor*1000*1000 + self.medium*1000 + self.minor;
    }


}


#[derive(Deserialize,Debug,Serialize)]
pub struct Device {
    pub id: i32,
    pub registration_date:chrono::DateTime<chrono::Utc>,

    pub role: String,
    pub phy_id: String,
    pub description: Option<String>,
    pub latest_version: i32,
    pub last_updated_timestamp: chrono::DateTime<chrono::Utc>,

}


#[derive(Deserialize,Debug,Serialize)]
pub struct Updates{
    pub id: i32,
    pub registration_date:chrono::DateTime<chrono::Utc>,

    pub version : i32,
    pub description: Option<String>,
    pub role: String,
}

#[derive(Debug)]
pub struct AppState{
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}



//check device update status
#[derive(Deserialize,Debug,Serialize)]
struct UpdateCheckMessage {
    pub phy_id: String,
    pub role: String,
    pub role_currentd_vesion: i32,
}


async fn device_check_updates(
    Json(payload):Json<UpdateCheckMessage>,
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode,HeaderMap,String),(StatusCode,HeaderMap,String)>{

    let mut response_headers = HeaderMap::new();
    let device_row_result = sqlx::query_as!(
        Device,
        "select * from devices where phy_id = $1 and role = $2",
        payload.phy_id, payload.role
    )
    .fetch_one(&db_pool).
    await;

    if device_row_result.is_err() {
       //Create a new device entry

    }

    Ok((       
        StatusCode::OK,
        response_headers,
        json!({"success": true, "data": device_row_result.unwrap()}).to_string()
    ))
}


