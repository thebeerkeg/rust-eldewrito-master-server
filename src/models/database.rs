use crate::models::announce::Announce;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Database {
    pub announces: Mutex<Vec<Announce>>
}

impl Database {
    pub fn new() -> Self {
        Database {
            announces: Mutex::new(Vec::new())
        }
    }

    pub fn add_announce(&self, announce: Announce) -> Result<(), ()> {
        self.announces.lock().unwrap().push(announce);
        Ok(())
    }
}
