use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct PlayerInfo {
    pub name: String,
    pub service_tag: String,
    pub primary_color: String,
    pub experience: Option<i64>
}
