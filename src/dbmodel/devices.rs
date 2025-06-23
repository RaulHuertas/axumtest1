
use serde::{Deserialize, Serialize};
use serde_json::json;



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






