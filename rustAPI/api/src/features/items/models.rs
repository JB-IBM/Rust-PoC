use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}  