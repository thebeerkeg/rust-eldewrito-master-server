use rocket::tokio::sync::Mutex;
use std::sync::Arc;
use crate::models::announce::Announce;

pub struct Database {
    pub announces: Arc<Mutex<Vec<Announce>>>
}

impl Database {
    pub fn new() -> Self {
        Database {
            announces: Arc::new(Mutex::new(Vec::new()))
        }
    }
}
