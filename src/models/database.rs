use crate::models::announce::Announce;
use std::sync::Mutex;
use crate::routes;
use std::collections::{HashMap};
use std::time::SystemTime;
use crate::models::server_info::ServerInfo;
use crate::utils::http_client;

// is set in the eldewrito client (30 + 2 * 60)
const ED_SERVER_CONTACT_TIME_LIMIT_SECS: u64 = 150;

// update server list on /list request when last updated > x secs
const SERVER_LIST_UPDATE_INTERVAL_SECS: u64 = 10;

#[derive(Debug)]
pub struct Database {
    announces: Mutex<Vec<Announce>>,
    server_list: Mutex<HashMap<String, SystemTime>>,
    server_list_last_updated: Mutex<SystemTime>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            announces: Mutex::new(Vec::new()),
            server_list: Mutex::new(HashMap::new()),
            server_list_last_updated: Mutex::new(SystemTime::now()),
        }
    }

    pub async fn handle_announce(&self, announce: Announce) -> routes::announce::Result {
        if announce.server.shutdown == Some(true) {
            self.server_list.lock().unwrap().remove(&*announce.server_addr());
            return routes::announce::Result {
                code: 0,
                msg: "Removed server from list.".to_string()
            }
        }

        // request server info
        let server_info_result = http_client::get::<ServerInfo>(format!("http://{}/", announce.server_addr())).await;

        if server_info_result.is_err() {
            return routes::announce::Result {
                code: 2,
                msg: "Failed to retrieve server info.".to_string()
            }
        }

        // you can do stuff with the server info if you want
        //let server_info = server_info_result.unwrap();

        self.server_list.lock().unwrap().insert(announce.server_addr(), announce.timestamp);

        routes::announce::Result {
            code: 0,
            msg: "OK".to_string()
        }
    }

    pub fn handle_list(&self) -> routes::list::Result {
        if self.server_list_last_updated.lock().unwrap().elapsed().unwrap().as_secs() > SERVER_LIST_UPDATE_INTERVAL_SECS {
            self.update_server_list();
        }

        routes::list::Result {
            code: 0,
            servers: self.server_list.lock().unwrap().keys().cloned().collect(),
            msg: "OK".to_string()
        }
    }

    fn update_server_list(&self) {
        // server should announce every 150 secs.
        // this removes all servers that haven't been re-announced in 300 secs.
        self.server_list.lock().unwrap().retain(|_, v| v.elapsed().unwrap().as_secs() < 2 * ED_SERVER_CONTACT_TIME_LIMIT_SECS);
        *self.server_list_last_updated.lock().unwrap() = SystemTime::now();
    }
}
