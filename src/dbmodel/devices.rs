
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::chrono::{DateTime,NaiveDateTime};

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


pub struct Device {
    pub id: String,
    pub registration_date:Option<NaiveDateTime>,

    pub role: String,
    pub phy_id: String,
    pub description: Option<String>,
    pub latest_version: i32,
    pub last_updated_timestamp: Option<NaiveDateTime>,

}


pub struct Updates{
    pub id: String,
    pub registration_date:Option<NaiveDateTime>,

    pub version : i32,
    pub description: Option<String>,
    pub role: String,
}

pub struct AppState{
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}





